from watcom_debug_info import try_get_watcom_debug_info

import pefile

import sys

if __name__ == '__main__':
    fname = sys.argv[1]
    pe = pefile.PE(fname)
    debug = try_get_watcom_debug_info(pe, fname)
    for x in debug:
        lo, hi = debug[x]
        print(f"0x{lo:00000000x}{'             ' if not hi else f' - 0x{hi:00000000x}'} {x}")