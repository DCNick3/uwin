
import argparse
import json
from pathlib import Path

from iced_x86 import Decoder, Instruction, FlowControl
from makeelf.elf import ELF
from makeelf.elfstruct import SHT, Elf32_Phdr, PF, Elf32_Shdr, SHF
import bitarray

# This file implements superset disassembly, similar to the one described in [1]
# It tries to shed obviously incorrect functions though, in order to simplify the work for lifter
# [1] Bauman, Erick, Zhiqiang Lin, and Kevin W. Hamlen. "Superset Disassembly: Statically Rewriting x86 Binaries Without Heuristics." NDSS. 2018.

parser = argparse.ArgumentParser()
parser.add_argument("--in_loaded_elf", type=Path, required=True,
  help="Path to elf file from the loader")

parser.add_argument("--out_addresses", type=Path,
  help="Path to output file that will get addresses of basic blocks possibly containing some code dumped to")

parser.add_argument("--out_elf", type=Path,
  help="Path to output elf file enriched with information on basic blocks possibly containing some code")

parser.add_argument('--verbose', '-v', action='store_true')

args = parser.parse_args()

def vprint(*args_, **kwargs):
  if args.verbose:
    print(*args_, **kwargs)

with open(args.in_loaded_elf, 'rb') as f:
  elf, _ = ELF.from_bytes(f.read())

segment: Elf32_Phdr
for segment in elf.Elf.Phdr_table[1:]:
  rwx = PF.PF_R | PF.PF_W | PF.PF_X
  # to ensure that there are no (unknown) semantic-changing flags
  assert segment.p_flags & rwx == segment.p_flags
  
  executable = segment.p_flags & PF.PF_X != 0
  if executable:
    # if executable - shouldn't be writable
    # SMC is not possible to handle really
    assert segment.p_flags & PF.PF_W == 0

shstr = elf.Elf.sections[elf.Elf.Ehdr.e_shstrndx]
def get_shstr(index):
  eindex = index
  while shstr[eindex] != 0:
    eindex += 1
  return shstr[index:eindex]

addresses = []

section: Elf32_Shdr
for i, section in enumerate(elf.Elf.Shdr_table[1:], 1):
  section_name = get_shstr(section.sh_name)
  rwx = SHF.SHF_ALLOC | SHF.SHF_WRITE | SHF.SHF_EXECINSTR
  # to ensure that there are no (unknown) semantic-changing flags
  assert section.sh_flags & rwx == section.sh_flags

  progbits = section.sh_type == SHT.SHT_PROGBITS
  executable = section.sh_flags & SHF.SHF_EXECINSTR != 0
  if progbits and executable:
    assert section.sh_flags & SHF.SHF_WRITE == 0 # should not be writable (SMC not supported)
    
    data = elf.Elf.sections[i]
    decoder = Decoder(32, data)
    def seek(offset):
      decoder.position = offset
      decoder.ip = section.sh_addr + offset
    instr = Instruction()

    visited = bitarray.bitarray(section.sh_size)
    visited.setall(0)

    valid = bitarray.bitarray(section.sh_size)
    valid.setall(0)

    def poke_at(offset):
      seek(offset)

      backtrack = []
      while True:
        p = decoder.position
        if decoder.can_decode:
          backtrack.append(p)
          if visited[p]:
            return
          visited[p] = 1
          valid[p] = 1

        decoder.decode_out(instr)

        fc = instr.flow_control
        break_bb = not (
            fc == FlowControl.NEXT or
            fc == FlowControl.CALL or
            fc == FlowControl.INDIRECT_CALL 
        )

        invalid = (
            # Note: this assumptions might not always be correct
            fc == FlowControl.INTERRUPT or
            fc == FlowControl.EXCEPTION
        )

        if invalid:
          #print(f"Cut {backtrack}")
          for b in backtrack:
            valid[b] = 0
          #print(f"{instr} @ 0x{decoder.ip:08x}")
          return
        elif break_bb:
          backtrack.clear()
    for i in range(0, section.sh_size):
      if not visited[i]:
        poke_at(i)

    valid_cnt = valid.count(1)
    size = section.sh_size
    vprint(f"{str(section_name):25}: {valid_cnt:10} / {size:10} addrs valid ({1 - valid_cnt/size:6.2%} pruned)")

    for offset in valid.itersearch(1):
      addresses.append(section.sh_addr + offset)

addresses.sort()

if args.out_addresses:
  with open(args.out_addresses, 'w') as f:
    for addr in addresses:
      f.write(f"0x{addr:x}\n")

if args.out_elf:
  cur_address = addresses[0]

  # use delta encoding to save some space
  delta_addresses = [cur_address]
  for x in addresses[1:]:
    delta_addresses.append(x - cur_address)
    cur_address = x

  data = ['uwin_disas_v1'] + delta_addresses
  data = bytes(json.dumps(data, separators=(',', ':')), 'utf8')
  elf._append_section(b'.uwin_bbaddrs', data, 0)
  
  with open(args.out_elf, 'wb') as f:
    f.write(bytes(elf))
