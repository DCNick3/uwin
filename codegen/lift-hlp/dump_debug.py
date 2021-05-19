from watcom_debug_info import try_get_watcom_debug_info

import pefile

import sys

if __name__ == '__main__':
    fname = sys.argv[1]
    pe = pefile.PE(fname)
    debug = try_get_watcom_debug_info(pe, fname)
    for x in debug:
        print(f"0x{debug[x]:00000000x} {x}")