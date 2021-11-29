
import argparse
from os import error
import tarfile
import sys
from pathlib import Path
from makeelf.elfsect import STT
from makeelf.elfstruct import PF, PT, SHF, SHT, Elf32_Phdr
import json

from pefile import PE, ImportData, SectionStructure
import makeelf.elf as elf

# this tool will take a number of dll files, one exe file and "load" them
# all the loading is done statically, generating one big self-contained elf file with custom metadata attached

parser = argparse.ArgumentParser()
parser.add_argument('--in_executables', type=Path, nargs='+',
  help="Executables to process (either EXE or DLL)")
parser.add_argument('--in_executable_tars', type=Path, nargs='+',
  help="Tars of executables to process")
parser.add_argument('--out_elf', type=Path, required=True,
  help="Path to loaded elf file to generate")
parser.add_argument('--keep-all', nargs='+',
  help="List of dll names to keep in memory even if the executable does not (transitively) depend on it. " \
    "It will also ensure that all its entries are available (that is, will load dependencies exported as forwarders). "\
    "The name does not correspond to filename, but to the value in DIRECTORY_ENTRY_EXPORT")
parser.add_argument('--exe-name', default='main.exe',
  help="Name of the main module for the module table")
parser.add_argument('--verbose', '-v', action='store_true')

args = parser.parse_args()

def eprint(*args, **kwargs):
  print(*args, file=sys.stderr, **kwargs)

def vprint(*args_, **kwargs):
  if args.verbose:
    print(*args_, **kwargs)


executables = list[PE]()

for exe in args.in_executables or []:
  executables.append(PE(exe))

for tar in args.in_executable_tars or []:
  with tarfile.TarFile(tar, 'r') as tar:
    member: tarfile.TarInfo
    for member in tar.getmembers():
      if member.isfile():
        data = tar.extractfile(member).read()
        executables.append(PE(data=data))

if len(executables) == 0:
  raise RuntimeError("No executables supplied")

exes = [ x for x in executables if x.is_exe() ]
assert len(exes) == 1, "Exactly one exe should be supplied to the loader! (%d found)" % len(exes)
the_exe = exes[0]

assert len([ x for x in executables if x.is_dll()]) == len(executables) - 1, "Some weird dlls? Dunno, lol..."

for pe in executables:
  if pe.is_dll() and hasattr(pe, 'DIRECTORY_ENTRY_EXPORT'):
    name = pe.get_string_at_rva(pe.DIRECTORY_ENTRY_EXPORT.struct.Name)
    assert name, "All dlls should have a name"
    pe.name = name
the_exe.name = bytes(args.exe_name, 'ascii')

assert len(set(x.name.lower() for x in executables)) == len(executables), "Some modules have the same name"

name2module = { x.name: x for x in executables }

def align_down(v, granularity):
  return v // granularity * granularity
def align_up(v, granularity):
  return (v + granularity - 1) // granularity * granularity

def perform_load():
  modules_to_load = list[PE]()
  loaded = set()
  loading = set()
  errors = []

  used_regions = []

  def find_address_and_relocate_there(mod: PE):
    LOAD_ALIGN = 0x10000
    SECTION_ALIGN = 0x1000
    def intersect(a, b):
      a1, a2 = a
      b1, b2 = b
      return a2 > b1 and b2 > a1
    # PERF: another linear search
    assert mod.OPTIONAL_HEADER.ImageBase % LOAD_ALIGN == 0
    assert mod.OPTIONAL_HEADER.SizeOfImage % SECTION_ALIGN == 0
    preferred_rg = (mod.OPTIONAL_HEADER.ImageBase, mod.OPTIONAL_HEADER.ImageBase + mod.OPTIONAL_HEADER.SizeOfImage)
    for rg in used_regions:
      if intersect(rg, preferred_rg):
        break
    else:
      vprint(f"Putting {mod.name} at its preferred loading address - 0x{preferred_rg[0]:08x}")
      used_regions.append(preferred_rg)
      used_regions.sort() # shit
      return mod
    
    base = align_up(used_regions[-1][1], SECTION_ALIGN)
    used_regions.append((base, base + mod.OPTIONAL_HEADER.SizeOfImage))
    used_regions.sort() # shit

    vprint(f"Relocating {mod.name} to 0x{base:08x}")
    mod.relocate_image(base)
    return mod

  def str_imp(imp: tuple[str, int]):
    if not imp[0]:
      return f"#{imp[1]}"
    return imp[0]

  def lookup_import(mod: PE, imp: tuple[str, int]):
    if not hasattr(mod, 'DIRECTORY_ENTRY_EXPORT'):
      errors.append(f"Cannot find an entry {str_imp(imp)} in a module {mod.name} (it exports nothing!)")
      return
    exports = mod.DIRECTORY_ENTRY_EXPORT
    # PERF: yay, linear search
    # I don't care about performance here anyways... At least yet
    for sym in exports.symbols:
      if imp[0] and imp[0] == sym.name or \
          imp[0] is None and imp[1] == sym.ordinal:
        assert not sym.forwarder # TODO
        return mod.OPTIONAL_HEADER.ImageBase + sym.address
    errors.append(f"Cannot find an entry {str_imp(imp)} in a module {mod.name}")


  def load(name) -> PE:
    if name not in name2module:
      errors.append("Cannot find module: " + str(name))
      return None
    mod = name2module[name]
    if name in loaded:
      return mod
    if name in loading:
      raise RuntimeError('DLL import loop detected')
    loading.add(mod.name)

    mod = find_address_and_relocate_there(mod)
      

    if hasattr(mod, 'DIRECTORY_ENTRY_IMPORT'):
      for dep in mod.DIRECTORY_ENTRY_IMPORT:
        name = mod.get_string_at_rva(dep.struct.Name)
        dep_mod = load(name)
        
        if not dep_mod:
          continue # this may happen if the user didn't supply the module
        # we don't want to fail right away to collect all the errors

        vprint(f"Binding imports of {mod.name} from {dep_mod.name}:")
        imp: ImportData
        for imp in dep.imports:
          # bind it!
          imp_addr = lookup_import(dep_mod, (imp.name, imp.ordinal))
          vprint(f" - bound {str(imp.name):20} -> 0x{imp_addr:08x} (thunk @ 0x{imp.address:08x})")
          assert mod.set_dword_at_rva(imp.address - mod.OPTIONAL_HEADER.ImageBase, imp_addr)

    loaded.add(mod.name)
    modules_to_load.append(mod)

    return mod

  load(the_exe.name)
  if args.keep_all:
    for name in args.keep_all:
      mod = load(bytes(name, 'ascii'))
      if hasattr(mod, 'DIRECTORY_ENTRY_EXPORT'):
        for sym in mod.DIRECTORY_ENTRY_EXPORT.symbols:
          lookup_import(mod, (None, sym.ordinal)) # import all syms by ordinals
  
  if errors:
    raise RuntimeError("Collected errors during dependency lookup:\n" + '\n'.join(errors))

  return modules_to_load

modules = perform_load()

vprint("Loaded modules:", [ x.name for x in modules ])

e = elf.ELF(e_machine=elf.EM.EM_386)

segment2section = dict[int,int]()

UWIN_DEBUG_TYPE = 0x6E697775

def enrich_debug(module: PE, pe_section2elf_section: dict[int, int]):
  found_uwin_debug = False

  # first try to add info harvested from uwin stubs
  if hasattr(module, 'DIRECTORY_ENTRY_DEBUG'):
    for debug in module.DIRECTORY_ENTRY_DEBUG:
      debug = debug.struct
      if debug.Type == UWIN_DEBUG_TYPE:
        found_uwin_debug = True
        vprint(f"Found uwin debug info for module {module.name}")
        # I didn't find the data by file pointer exposed =(
        data = module.__data__[debug.PointerToRawData:debug.PointerToRawData+debug.SizeOfData]
        data = json.loads(data)
        
        # check if the data has expected format marker
        assert data[0] == "uwin_stub_v1"
        data = data[1:]

        for offset, size, name, is_data in data:
          addr = module.OPTIONAL_HEADER.ImageBase + offset
          name = bytes(name, 'ascii')
          # just so happens that uwin stubs have only valid ascii in them (because json)
          # but for dlls we don't control this might not be the case
          full_name = module.name + b'!' + name

          pe_sec = module.get_section_by_rva(offset)
          pe_sec_id = [ i for i, x in enumerate(module.sections) if x == pe_sec ][0]
          elf_sec_id = pe_section2elf_section[pe_sec_id]

          type = STT.STT_OBJECT if is_data else STT.STT_FUNC

          e.append_symbol(full_name, elf_sec_id, addr, size, sym_type=type)
          pass

        #vprint(data)
        #symname = module.name + b"!" + 
  
  # if it's not our stub - just add info about exported symbols & thunks
  if not found_uwin_debug:
    pass
    # TODO

for module in sorted(modules, key=lambda x: x.OPTIONAL_HEADER.ImageBase):
  base = module.OPTIONAL_HEADER.ImageBase

  pe_section2elf_section = dict[int, int]()

  class dotdict(dict):
    """dot.notation access to dictionary attributes"""
    __getattr__ = dict.get
    __setattr__ = dict.__setitem__
    __delattr__ = dict.__delitem__

  # create a fake header section
  header_section = dotdict()
  header_section.Name = b'.pehdr'
  header_section.IMAGE_SCN_MEM_READ = True
  header_section.IMAGE_SCN_MEM_WRITE = False
  header_section.IMAGE_SCN_MEM_EXECUTE = False
  header_section.VirtualAddress = 0
  header_section.Misc_VirtualSize = len(pe.header)
  header_section.get_data = lambda: pe.header

  pe_section: SectionStructure
  for pe_section_index, pe_section in enumerate([header_section] + module.sections, -1): # the fake header section has index -1
    new_name = (module.name + pe_section.Name).rstrip(b'\x00')

    r = pe_section.IMAGE_SCN_MEM_READ
    w = pe_section.IMAGE_SCN_MEM_WRITE
    x = pe_section.IMAGE_SCN_MEM_EXECUTE
    access = ('r' if r else '') + ('w' if w else '') + ('x' if x else '')

    def rwx_mask(r_v, w_v, x_v):
      return (r_v if r else 0) | (w_v if w else 0) | (x_v if x else 0)

    addr = base + pe_section.VirtualAddress
    virt_size = align_up(pe_section.Misc_VirtualSize, 0x1000)
    data = pe_section.get_data()

    # Ewwww, that's ugly...
    # the public functions are not strong enough though
    sec_id = e._append_section(new_name, data, addr,
                sh_type=SHT.SHT_PROGBITS, sh_flags=SHF.SHF_ALLOC | rwx_mask(0, SHF.SHF_WRITE, SHF.SHF_EXECINSTR), 
                sh_link=0, sh_info=0,
                sh_addralign=0x1000, sh_entsize=0)

    pe_section2elf_section[pe_section_index] = sec_id

    phdr_flags = rwx_mask(PF.PF_R, PF.PF_W, PF.PF_X)

    segment2section[len(e.Elf.Phdr_table)] = sec_id

    e.Elf.Phdr_table.append(Elf32_Phdr(p_type=PT.PT_LOAD, p_offset=0, p_vaddr=addr,
                p_paddr=0, p_filesz=len(data), p_memsz=virt_size,
                p_flags=phdr_flags, p_align=0x1000, little=e.little))

    # !! KLUDGE: makeelf does not handle p_offset of Elf32_Phdr at all
    # that's why mapping from segments to sections is saved to later fix the shit

    vprint(f"elf section {str(new_name):24} @ 0x{addr:08x} ({virt_size:08x}) [{access:3}] -> {sec_id}")
  
  enrich_debug(module, pe_section2elf_section)

e.Elf.Ehdr.e_entry = the_exe.OPTIONAL_HEADER.ImageBase + the_exe.OPTIONAL_HEADER.AddressOfEntryPoint

# update section offsets...
bytes(e)
# ... and point the segments to them
for i, h in enumerate(e.Elf.Phdr_table):
  if i == 0:
    continue
  offset = e.Elf.Shdr_table[segment2section[i]].sh_offset
  h.p_offset = offset

# now we (finally) can save the resulting elf file

with open(args.out_elf, 'wb') as f:
  f.write(bytes(e))
