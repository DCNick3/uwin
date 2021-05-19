
import struct
import subprocess
import tempfile
import re
from pathlib import Path

# master_dbg_header from https://github.com/open-watcom/open-watcom-v2/blob/047386398d3b538c8ba9792714e6143b1f44f0ca/bld/watcom/h/wdbginfo.h#L101
master_dbg_header_format = "<HBBBBHHI"
# another header from https://github.com/open-watcom/open-watcom-v2/blob/047386398d3b538c8ba9792714e6143b1f44f0ca/bld/exedump/c/wdwarf.c#L53
debug_header_format = "<4sIII"

WAT_DBG_SIGNATURE = 0x8386

TIS_SIGNATURE = b'TIS\x00'

DEBUG_HEADER_SIZE = struct.calcsize(debug_header_format)


READELF_DWARF_REGEX = re.compile(r" <\d+><[a-f0-9]+>: Abbrev Number: \d+ \(([a-zA-Z_]+)\)\n((?:\s+<[a-f0-9]+>\s+[a-zA-Z_]+\s*:[^\n]*\n)+)")
READELF_DWARF_DIE_ATTR_REGEX = re.compile(r"\s+<[a-f0-9]+>\s+([a-zA-Z_]+)\s*: ([^ \n]+)[^\n]*\n")


# header detection logic from https://github.com/open-watcom/open-watcom-v2/blob/047386398d3b538c8ba9792714e6143b1f44f0ca/bld/exedump/c/wdwarf.c
def try_get_watcom_debug_info(pe, filename) -> None or list:
    with open(filename, 'rb') as f:

        f.seek(-DEBUG_HEADER_SIZE, 2)
        candidate_header_data = f.read()

        signature, vendor_id, info_type, info_size = struct.unpack(debug_header_format, candidate_header_data)

        if signature != TIS_SIGNATURE:
            return dict()

        f.seek(-info_size, 2)

        # we could have used pyelftools, but openwatcom seems to generate somewhat broken DWARF info
        #       and pyelftools doesn't handle it well
        # so, instead we rely on readelf and parse it's output :shrug:

        with tempfile.TemporaryDirectory() as d:
            dpath = Path(d)
            debug_info_elf_filename = dpath / 'debug_info.elf'
            with open(debug_info_elf_filename, 'wb') as df:
                df.write(f.read()[:-DEBUG_HEADER_SIZE])
            output = subprocess.check_output(['readelf', '-wi', debug_info_elf_filename], text=True,
                                             # to silence warnings
                                             stderr=subprocess.DEVNULL)

        dies = READELF_DWARF_REGEX.findall(output)

        funs = dict()

        pc_offset = pe.OPTIONAL_HEADER.ImageBase + pe.OPTIONAL_HEADER.BaseOfCode

        for tag, attrs in dies:
            if tag == 'DW_TAG_label' or tag == 'DW_TAG_subprogram':
                attrs_dict = dict(READELF_DWARF_DIE_ATTR_REGEX.findall(attrs))
                funs[attrs_dict['DW_AT_name']] = pc_offset + int(attrs_dict['DW_AT_low_pc'], 16)

        return funs

