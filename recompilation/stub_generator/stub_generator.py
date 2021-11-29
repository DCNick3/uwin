
import argparse
import json
import struct
import os
import tarfile
import shutil
import sys
from io import BytesIO
from collections import OrderedDict
from pathlib import Path
from pprint import pprint
from contextlib import contextmanager
from typing import IO, BinaryIO, Union


import pyparsing
import pefile

from def_parser import DefExportsEntry, DefFile, parse_def_string

# IN:
#  - list of .def files describing the stubs to generate (format is a subset of MSVC module definition file)
# OUT:
#  - generated dlls (probably inside some archive, as bazel limits generation of multiple files)
#  - a mapping between functions and their ids (basically adresses used in CALL FAR instructions)
#  - function rvas: a mapping between function and and rva (Relative Virtual Address) for each dll

# the generated dlls contain stubs that use far call instructions to call into uwin native code
# they also contain custom debug information, specifying regions for the stubs and their type

# TODO: ordinal-oriented libraries (like winsock)
# probably want to take a .def file

parser = argparse.ArgumentParser()

parser.add_argument("--in_module_definitions", type=Path, nargs='+',
  required=True,
  help="Paths to .def files describing the stubs to be generated")

parser.add_argument("--out_stubs_directory", type=Path,
  help="A directory to place the generated dll stubs to.")
parser.add_argument("--out_stubs_tar", type=Path,
  help="A path to tar file to place the generated dll stubs to. This is the same as --out_stubs_directory, but tars all the dlls")

parser.add_argument("--out_fun_mapping_json", type=Path,
  help="A path to json file that will contain mapping between functions and their ids (basically adresses used in CALL FAR instructions)")


args = parser.parse_args()

dlls: dict[str, DefFile] = dict()

for def_file in args.in_module_definitions:
  with open(def_file) as f:
    try:
      def_file = parse_def_string(f.read())
    except pyparsing.ParseBaseException as e:
      print("While parsing", def_file)
      print(e.explain())
      sys.exit(1)
  def_file['library_name'] = def_file.library_name.lower()

  assert def_file.library_name.endswith('.dll')

  if def_file.library_name in dlls:
    raise RuntimeError("Duplicate dll name: " + def_file.library_name)

  dlls[def_file.library_name] = def_file


MAPPING_BASE = 0x120000
UWIN_MAGIC_SEGMENT = 0x7775

UWIN_DEBUG_TYPE = 0x6e697775

next_mapped_id = MAPPING_BASE

mapping = dict[str, int]()
mapping_per_dll: dict[str, dict[str, int]] = dict()
for dll in dlls.values():
  for entry in dll.exports.values():
    if not (entry.redirect_to or entry.data):
      mapping[(dll.library_name, entry.entryname)] = next_mapped_id
      mapping_per_dll.setdefault(dll.library_name, dict())[entry.entryname] = next_mapped_id
      next_mapped_id = next_mapped_id + 1
inv_mapping = {v: k for k, v in mapping.items()}

assert len(mapping) == len(inv_mapping)

class FunctionInfo(dict):
  def __init__(self, offset, size, is_data):
    dict.__init__(self,
      offset=offset,
      size=size,
      is_data=is_data)
  
  @property
  def offset(self) -> int: return self['offset']
  @property
  def size(self) -> int: return self['size']
  @property
  def is_data(self) -> bool: return self['is_data']

def generate_code(dll_def: DefFile, mapping: dict[str, int]) -> tuple[bytes, dict[str, FunctionInfo]]:
  out = BytesIO()
  function_info = dict[str, FunctionInfo]()

  for export in dll_def.exports.values():
    sta = out.tell()
    if export.data:
      out.write(export.data)
    elif not export.redirect_to:
      fun_id = mapping[export.entryname]
      # far call
      out.write(b'\x9a' + struct.pack('<IH', fun_id, UWIN_MAGIC_SEGMENT))
      # ret
      out.write(b'\xc3')
    fin = out.tell()

    function_info[export.entryname] = FunctionInfo(sta, fin - sta, not not export.data )

  return out.getvalue(), function_info
  

def generate_pe(code: bytes, dll_def: DefFile, functions_info: dict[str, FunctionInfo], out: BinaryIO):
  # some helper routines
  def pack_u8(v):
    return struct.pack('<B', v)
  def pack_u16(v):
    return struct.pack('<H', v)
  def pack_u32(v):
    return struct.pack('<I', v)

  def u8(v):
    out.write(pack_u8(v))
  def u16(v):
    out.write(pack_u16(v))
  def u32(v):
    out.write(pack_u32(v))
  def tell():
    return out.tell()
  def seek(p):
    return out.seek(p)

  def align_down(v, granularity):
    return v // granularity * granularity
  def align_up(v, granularity):
    return (v + granularity - 1) // granularity * granularity

  functions = dll_def.exports

  SizeOfOptionalHeader = 28 + 68 + 16 * 8
  HeadersSizeInFile = 0x400
  HeadersSizeInMemory = 0x1000
  CodeBase = HeadersSizeInMemory

  FileAlignment = 0x200
  SectionAlignment = 0x1000

  DebugInfoEntries = 1

  @contextmanager
  def devnull():
    nonlocal out
    saved_out = out
    out = BytesIO()
    yield out
    out = saved_out

  def generate_export_table(export_table_rva: int):
    # generally the layout is as following:
    # - header (fixed size)
    # - function pointers (size proportional to # of ordinal slots)
    # - export name pointers (proportional to # of named functions)
    # - export ordinal values (proportional size)
    # - strings: export names & dll name (unknown size)
    # we build strings section greedily while outputting all the other structures
    # we can calculate strings RVA even before outputting them because the sizes of all other sections can be calculated in advance

    export_table_start = tell()

    fixed_ordinals = [ x.ordinal for x in functions.values() if x.ordinal ]
    assert len(fixed_ordinals) == len(set(fixed_ordinals))

    ordinal_assignment: dict[int, DefExportsEntry] = {x.ordinal: x for x in functions.values() if x.ordinal}
    inv_ordinal_assignment: dict[int, int] = dict()
    free_ordinal = 1
    for i, fun in enumerate(functions.values()):
      if fun.ordinal:
        inv_ordinal_assignment[i] = fun.ordinal
        continue
      while free_ordinal in ordinal_assignment:
        free_ordinal += 1
      ordinal_assignment[free_ordinal] = fun
      inv_ordinal_assignment[i] = free_ordinal
    
    ordinal_base = min(ordinal_assignment)

    number_of_ordinals = max(ordinal_assignment) - ordinal_base + 1
    number_of_names = len([x for x in functions.values() if x.export_by_name])


    string_positions = dict()
    strings_builder = BytesIO()

    export_address_table_offset = 40
    name_pointer_table_offset = 40 + 4 * number_of_ordinals
    ordinal_table_offset = 40 + 4 * number_of_ordinals + 4 * number_of_names
    strings_offset = 40 + 4 * number_of_ordinals + 4 * number_of_names + 2 * number_of_names

    def put_str(s: str) -> int:
      if s in string_positions:
        return string_positions[s]
      r = export_table_rva + strings_offset + strings_builder.tell()
      string_positions[s] = r
      strings_builder.write(bytes(s, 'utf8') + b'\0')
      return r

    u32(0) # Export Flags (Reserved, must be 0)
    u32(0) # Time/Date Stamp
    u16(0) # Major Version
    u16(0) # Minor Version
    u32(put_str(dll_def.library_name)) # Name RVA
    u32(ordinal_base) # Ordinal Base
    u32(number_of_ordinals) # Address Table Entries
    u32(number_of_names) # Number of Name Pointers
    u32(export_address_table_offset + export_table_rva) # Export Address Table RVA 
    u32(name_pointer_table_offset + export_table_rva) # Name Pointer RVA 
    u32(ordinal_table_offset + export_table_rva) # Ordinal Table RVA

    assert tell() - export_table_start == export_address_table_offset

    for ordinal in range(ordinal_base, ordinal_base + number_of_ordinals):
      if ordinal not in ordinal_assignment:
        u32(0)
        continue
      function = ordinal_assignment[ordinal]
      if function.redirect_to:
        u32(put_str('.'.join(function.redirect_to)))
      else:
        u32(CodeBase + functions_info[function.entryname].offset)

    assert tell() - export_table_start == name_pointer_table_offset

    names = list(sorted((fun.entryname, ordinal) for ordinal, fun in ordinal_assignment.items() if fun.export_by_name))
    
    for entry_name, _ in names:
      u32(put_str(entry_name))

    assert tell() - export_table_start == ordinal_table_offset

    for _, ordinal in names:
      u16(ordinal - ordinal_base)

    assert tell() - export_table_start == strings_offset

    out.write(strings_builder.getvalue())

  def generate_uwin_debug_info():
    res = []
    res.append("uwin_stub_v1")
    for name, function in functions_info.items():
      res.append([CodeBase+function.offset, function.size, name, function.is_data])
    return bytes(json.dumps(res), 'utf8')

  def output_debug_info(debug_info_offset: int, debug_info_directory_size: int):
    debug_info = generate_uwin_debug_info()

    debug_info_start = tell()

    # first - output the directory information
    u32(0) # Characteristics
    u32(0) # TimeDateStamp
    u16(0) # MajorVersion
    u16(0) # MinorVersion
    u32(UWIN_DEBUG_TYPE) # Type
    u32(len(debug_info)) # SizeOfData
    u32(0) # AddressOfRawData (not loaded, so no address)
    u32(debug_info_offset + debug_info_directory_size) # PointerToRawData

    assert tell() - debug_info_start == debug_info_directory_size
    
    out.write(debug_info)


  def output_headers(export_table_size: int, debug_directory_size: int):
    # first goes DOS header & DOS stub with PE header offset at 0x3C
    # just write out the hard-coded DOS program =)
    # as for PE header offset - it points directly afterwards itself
    DOS_STUB = b'MZ\x90\x00\x03\x00\x00\x00\x04\x00\x00\x00\xff\xff\x00\x00\xb8\x00\x00\x00\x00\x00\x00\x00@\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x0e\x1f\xba\x0e\x00\xb4\t\xcd!\xb8\x01L\xcd!This program cannot be run in DOS mode.\r\r\n$\x00\x00\x00\x00\x00\x00\x00\xc0\xe4?\xa6\x84\x85Q\xf5\x84\x85Q\xf5\x84\x85Q\xf5~\xa1\x11\xf5\xa9\x85Q\xf5~\xa1M\xf5\x0b\x85Q\xf5^\xa6M\xf5\x86\x85Q\xf5~\xa6H\xf5\x8c\x85Q\xf5\x84\x85P\xf5+\x85Q\xf5\xfa\xa7L\xf5\xc7\x85Q\xf5\xb7\xa7t\xf5\x8e\x85Q\xf5\xfa\xa7M\xf5\x87\x85Q\xf5~\xa1L\xf5\xb4\x85Q\xf5~\xa1\x14\xf5\x85\x85Q\xf5~\xa1l\xf5\x85\x85Q\xf5Rich\x84\x85Q\xf5\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'

    headers_start = tell()

    assert HeadersSizeInMemory % SectionAlignment == 0

    out.write(DOS_STUB)

    # COFF Header
    u32(pefile.IMAGE_NT_SIGNATURE)
    u16(pefile.MACHINE_TYPE["IMAGE_FILE_MACHINE_I386"]) # Machine 
    u16(1) # NumberOfSections 
    # we are gonna create only one .text (r-x) section that will hold everything, so this should be fine

    u32(0) # TimeDateStamp  (unix time)
    u32(0) # PointerToSymbolTable
    u32(0) # NumberOfSymbolTable
    u16(SizeOfOptionalHeader) # SizeOfOptionalHeader (size of Standard fields + size of Windows-specific fields + size of Data directories)
    u16(0x2102) # Characteristics (IMAGE_FILE_EXECUTABLE_IMAGE = 0x2 | IMAGE_FILE_32BIT_MACHINE = 0x100 | IMAGE_FILE_DLL = 0x2000)
    
    opt_hdr_start = tell()

    # Optional header
    u16(pefile.OPTIONAL_HEADER_MAGIC_PE)

    # I don't think anyone cares about these
    u8(0) # MajorLinkerVersion
    u8(0) # MinorLinkerVersion
    u32(0) # SizeOfCode
    u32(0) # SizeOfInitializedData
    u32(0) # SizeOfUninitializedData

    u32(0) # AddressOfEntryPoint (0 -> no entrypoint)

    u32(CodeBase) # BaseOfCode (just after the page with the header)
    u32(0) # BaseOfData used?? TODO


    u32(0x10000000) # ImageBase
    u32(SectionAlignment) # SectionAlignment
    u32(FileAlignment) # FileAlignment 

    # For some reason, I always hated PE file format...
    u16(1) # MajorOperatingSystemVersion
    u16(11) # MinorOperatingSystemVersion
    u16(0) # MajorImageVersion
    u16(0) # MinorImageVersion
    u16(4) # MajorSubsystemVersion
    u16(0) # MinorImageVersion
    u32(0) # Win32VersionValue (Reserved, must be zero)
    u32(HeadersSizeInMemory + \
        align_up(len(code) + export_table_size + debug_directory_size, SectionAlignment)
      ) # SizeOfImage
    u32(0x400) # SizeOfHeaders (I hope it's enough)
    u32(0) # CheckSum (nah, that's useless)
    u16(2) # Subsystem (IMAGE_SUBSYSTEM_WINDOWS_GUI)
    u16(0x140) # (IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE = 0x40 | IMAGE_DLLCHARACTERISTICS_NX_COMPAT = 0x100)

    # does this even make sense for a dll?... (aaaaaaaaa)
    u32(0x100000) # SizeOfStackReserve
    u32(0x1000) # SizeOfStackCommit
    u32(0) # SizeOfHeapReserve
    u32(0) # SizeOfHeapCommit
    u32(0) # LoaderFlags (Reserved, must be zero.)
    u32(16) # NumberOfRvaAndSizes (# of data-directory entries); we fill out all for extra measure

    directories = [ (0, 0) for x in range(0, 16)]

    # Export Table Data Directory
    directories[0] = (CodeBase + len(code), export_table_size)

    # Debug
    directories[6] = (CodeBase + len(code) + export_table_size, debug_directory_size)

    for rva, size in directories:
      u32(rva); u32(size)

    opt_hdr_end = tell()

    assert opt_hdr_end - opt_hdr_start == SizeOfOptionalHeader

    # Section table
    # the only section - .text
    # it will contain both code, export table & debug info
    out.write(b'.text\0\0\0') # Name (8 bytes)
    u32(align_up(len(code) + export_table_size + debug_directory_size, SectionAlignment)) # VirtualSize
    u32(HeadersSizeInMemory) # VirtualAddress
    u32(align_up(len(code) + export_table_size + debug_directory_size, FileAlignment)) # SizeOfRawData
    u32(HeadersSizeInFile) # PointerToRawData
    u32(0) # PointerToRelocations (section relocs are not for us)
    u32(0) # PointerToLinenumbers
    u16(0) # NumberOfRelocations
    u16(0) # NumberOfLinenumbers
    u32(0x60000020) # Characteristics (IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_READ)

    # pad with zeroes up be aligned up to FileAlignment
    pad_size = align_up(tell() - headers_start, FileAlignment) - (tell() - headers_start)
    out.write(b'\0' * pad_size)

    headers_end = tell()

    assert headers_end - headers_start == HeadersSizeInFile

  with devnull() as tmp:
    generate_export_table(0)
  
  file_start = tell()

  export_table_size = len(tmp.getbuffer())
  debug_info_directory_size = DebugInfoEntries * 28

  code_offset = HeadersSizeInFile
  export_table_offset = code_offset + len(code)
  debug_info_offset = code_offset + len(code) + export_table_size

  output_headers(export_table_size, debug_info_directory_size)

  assert tell() - file_start == code_offset

  out.write(code)

  assert tell() - file_start == export_table_offset

  generate_export_table(CodeBase + len(code))

  assert tell() - file_start == debug_info_offset

  output_debug_info(debug_info_offset, debug_info_directory_size)

  pad_size = align_up(tell() - file_start, FileAlignment) - (tell() - file_start)
  out.write(b'\0' * pad_size)

def generate_dll(dll_def: DefFile, mapping: dict[str, int], out: BinaryIO):
  code, offsets = generate_code(dll_def, mapping)

  generate_pe(code, dll_def, offsets, out)

if args.out_stubs_tar:
  tar = tarfile.TarFile(args.out_stubs_tar, 'w')
  
@contextmanager
def get_output(library_name) -> BinaryIO:
  if not (args.out_stubs_directory or args.out_stubs_tar):
    raise RuntimeError("No output selected (use either --out_stubs_directory or --out_stubs_tar)")
  
  dll_io = BytesIO()

  yield dll_io

  if args.out_stubs_directory:
    dll_io.seek(0)
    os.makedirs(args.out_stubs_directory, exist_ok=True)
    with open(args.out_stubs_directory / library_name, 'wb') as f:
      shutil.copyfileobj(dll_io, f)
  if args.out_stubs_tar:
    dll_io.seek(0)
    fileinfo = tarfile.TarInfo('dlls/' + library_name)
    fileinfo.size = len(dll_io.getbuffer())
    tar.addfile(fileinfo, dll_io)

for dll in dlls.values():
  with get_output(dll.library_name) as dll_io:
    generate_dll(dll, mapping_per_dll[dll.library_name], dll_io)