from os import path
import sys
import tempfile
from pathlib import Path
import subprocess
import argparse

import pefile
from yaspin import yaspin

from watcom_debug_info import try_get_watcom_debug_info


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


GHIDRA_DIR = Path('/opt/ghidra')
GHIDRA_ANALYZE_HEADLESS = GHIDRA_DIR / 'support' / 'analyzeHeadless'
GHIDRA_SCRIPTS_PATH = Path(path.dirname(__file__)) / 'ghidra_scripts'
GET_BBS_SCRIPT_PATH = GHIDRA_SCRIPTS_PATH / 'GetBBS.java'

UWIN_LIFT_PATH = Path(
    '/home/dcnick3/trash/homm3-switch/code/lifting/remill/cmake-build-relwithdebinfo-clang/bin/uwin-lift/uwin-lift-11.1')
UWIN_CLANG_PATH = Path(
    '/home/dcnick3/trash/homm3-switch/code/lifting/remill/cmake-build-relwithdebinfo-clang/remill-clang-11.1')

SHOW_SPINNERS = True


def spin(*args, **kwargs):
    if not SHOW_SPINNERS:
        class noop_spinner:
            def __getattr__(self, item):
                return self

            def __enter__(self):
                return self

            def __exit__(self, exc_type, exc_val, exc_tb):
                pass

            def ok(self, *args):
                pass

        return noop_spinner()
    return yaspin(*args, **kwargs)

# Maybe implement caching? For large executables this process can take quite some time...

def ghidralize(exe_name, extra_code_addresses):
    with spin(text='ghidralizing...', timer=True).white.bold.shark.on_red as spinner:
        with tempfile.TemporaryDirectory() as d:
            dpath = Path(d)
            extra_code_filename = dpath / 'extra_code.txt'
            with open(extra_code_filename, 'w') as f:
                f.write('\n'.join(map(str, extra_code_addresses)))
            bbs_filename = dpath / 'bbs.txt'

            try:
                subprocess.check_output([str(GHIDRA_ANALYZE_HEADLESS),
                                         dpath, 'ghidra_project',
                                         '-import', exe_name,
                                         '-deleteProject',
                                         '-postScript', GET_BBS_SCRIPT_PATH, bbs_filename, extra_code_filename,
                                         '-scriptPath', GHIDRA_SCRIPTS_PATH
                                         ], cwd=dpath, stderr=subprocess.STDOUT, text=True)
            except subprocess.CalledProcessError as exc:
                eprint('Ghidra output:\n' + exc.output)
                raise

            with open(bbs_filename) as f:
                res = [int(x) for x in f.read().split('\n') if x]
            res.sort()

            spinner.ok("✓")

            return res


def extract_code_and_debug_info(exe_name):
    with spin(text='extracting code and debug info...', timer=True).dqpb.white.on_green as spinner:
        pe = pefile.PE(exe_name)
        try:
            # TODO: come up with some better logic (probably needs some work in remill-lift)
            text_sections = [x for x in pe.sections if x.Name == b'.text\x00\x00\x00' or x.IMAGE_SCN_MEM_EXECUTE]
            assert len(text_sections) == 1
            text_section = text_sections[0]

            addr = pe.OPTIONAL_HEADER.ImageBase + text_section.VirtualAddress
            data = text_section.get_data()

            debug_info = try_get_watcom_debug_info(pe, exe_name)

            extra_code_addresses = list(set(debug_info.values()))
            name_map = {v: k for k, v in debug_info.items()}

            spinner.ok("✓")
            return (addr, data, extra_code_addresses, name_map)
        finally:
            pe.close()


def recompile(ir_path, o_path):
    with spin(text='recompiling...', timer=True).material.white.on_yellow as spinner:
        try:
            subprocess.check_output([str(UWIN_CLANG_PATH),
                                     Path(ir_path).absolute(), '-c', '-O3',
                                     '-o', Path(o_path).absolute()
                                     ], stderr=subprocess.STDOUT, text=True)

            spinner.ok("✓")
        except subprocess.CalledProcessError as exc:
            eprint('uwin-clang output:\n' + exc.output)
            raise


def lift(code_bin_path, code_address, bbs_path, name_map_path, ir_path):
    with spin(text='lifting...', timer=True).noise.white.bold.on_blue as spinner:
        try:
            subprocess.check_output([str(UWIN_LIFT_PATH),
                                     '--code_filename', Path(code_bin_path).absolute(),
                                     '--code_address', str(code_address),
                                     '--basic_blocks_filename', Path(bbs_path).absolute(),
                                     '--name_map_filename', Path(name_map_path).absolute(),
                                     '--ir_out', Path(ir_path).absolute()
                                     ], stderr=subprocess.STDOUT, text=True)

            spinner.ok("✓")
        except subprocess.CalledProcessError as exc:
            eprint('uwin-lift output:\n' + exc.output)
            raise


def do_the_thing(exe_name, extra_code_addresses, o_path):
    with tempfile.TemporaryDirectory() as d:
        dpath = Path(d)
        code_path = dpath / 'code.bin'
        bbs_path = dpath / 'bbs.txt'
        nm_path = dpath / 'nm.txt'
        ir_path = dpath / 'ir_out.ll'

        code_addr, code_data, extra_code_addresses_1, name_map = extract_code_and_debug_info(exe_name)
        bbs = ghidralize(exe_name, extra_code_addresses_1 + extra_code_addresses)

        with open(bbs_path, 'w') as f:
            f.write('\n'.join(map(str, bbs)))
        with open(nm_path, 'w') as f:
            f.write('\n'.join(map(lambda x: ' '.join(map(str, x)), name_map.items())))
        with open(code_path, 'wb') as f:
            f.write(code_data)

        lift(code_path, code_addr, bbs_path, nm_path, ir_path)
        recompile(ir_path, o_path)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(prog='lift-hlp', description='Convert a exe file into a lifted object file ready '
                                                                  'to link with uwin')

    parser.add_argument('exe_path')
    parser.add_argument('output_path')
    parser.add_argument('--extra-code-addresses-file')
    parser.add_argument('--silent', action='store_true')

    args = parser.parse_args()

    if args.silent:
        SHOW_SPINNERS = False

    if args.extra_code_addresses_file:
        with open(args.extra_code_addresses_file, 'r') as f:
            extra_code_addresses = [int(x) for x in f.read().split('\n')]
    else:
        extra_code_addresses = []
    do_the_thing(args.exe_path, extra_code_addresses, args.output_path)
    # print(ghidralize(EXE_FILE, []))
