
import argparse
import tarfile
from pathlib import Path

from pefile import PE
import makeelf.elf as elf

# this tool will take a number of dll files, one exe file and "load" them
# all the loading is done statically, generating one big self-contained elf file with custom metadata attached

parser = argparse.ArgumentParser()
parser.add_argument('--in_executables', type=Path, nargs='+',
  help="Executables to process (either EXE or DLL)")
parser.add_argument('--in_executable_tars', type=Path, nargs='+',
  help="Tars of executables to process")

args = parser.parse_args()

executables = []

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



e = elf.ELF(e_machine=elf.EM.EM_386)

k32_text_id = e.append_section('kernel32.dll.text', b'TEXTTEXTTEXTTEXT', 0x10000000)

e.append_symbol("kernel32_dll__GetLastError", k32_text_id, 4, 4)

e.append_segment(k32_text_id, flags='r-x')

print(bytes(e))
