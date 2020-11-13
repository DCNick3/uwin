
import argparse
import subprocess
import os
import os.path

from pprint import pprint

from mako.template import Template

from pycparserext.ext_c_parser import GnuCParser, FuncDeclExt

from pycparser.c_ast import Typedef, Decl

from model import FUN

DIR = os.path.dirname(os.path.realpath(__file__))
WINTYPES_H = DIR + "/wintypes.h"
INTERFACE_H_TEMPLATE = DIR + "/interface.h.mako"
INTERFACE_CPP_TEMPLATE = DIR + "/interface.cpp.mako"
DLL_CTX_H_TEMPLATE = DIR + "/dll_ctx.h.mako"
DISPATCHER_CPP_TEMPLATE = DIR + "/dispatcher.cpp.mako"

INTERFACE_H_OUT_FILE_TEMPLATE = 'win32/dll/%s_iface.h'
INTERFACE_CPP_OUT_FILE_TEMPLATE = 'win32/dll/%s_iface.cpp'
DLL_CTX_H_OUT_FILE = 'ctx/dll.h'
DISPATCHER_CPP_OUT_FILE = 'win32/dll/dispatcher.cpp'

parser = argparse.ArgumentParser()
parser.add_argument("--print_out_files", action="store_true")
parser.add_argument("bin_dir", type=str)
parser.add_argument("filenames", type=str, nargs='+')

interface_h_template = Template(filename=INTERFACE_H_TEMPLATE)
interface_cpp_template = Template(filename=INTERFACE_CPP_TEMPLATE)
dll_ctx_h_template = Template(filename=DLL_CTX_H_TEMPLATE)
dispatcher_cpp_template = Template(filename=DISPATCHER_CPP_TEMPLATE)


def preprocess(in_text):
    #print(['cpp', '-nostdinc', '-include', WINTYPES_H])
    return subprocess.check_output(['cpp', '-nostdinc', '-I', DIR, '-include', 'wintypes.h'], input=in_text, text=True)

def get_dllname(h_filename):
    return os.path.splitext(os.path.basename(h_filename))[0]

def parse_dll(h_filename):
    parser = GnuCParser()

    with open(h_filename) as f:
        text = preprocess(f.read())

    result = parser.parse(text)

    funs = []

    for x in result:
        if type(x) is Typedef:
            pass
        elif type(x) is Decl:
            if type(x.type) is FuncDeclExt:
                funs.append(FUN(x.type, x.funcspec))
            else:
                raise RuntimeError("Unsopported declaration type: " + str(type(x.type)))
        else:
            raise RuntimeError("Unsopported AST node: " + str(type(x)))
        #print(x)

    return funs

def render_dll(dll_name, funs):

    data = {
        'funs': funs,
        'dll_name': dll_name,
    }

    interface_h = interface_h_template.render(**data)
    interface_cpp = interface_cpp_template.render(**data)

    return (interface_h, interface_cpp)

def parse_dlls(filenames):
    res = dict()
    index = 0x80000000
    for x in filenames:
        funs = list(enumerate(parse_dll(x), index))
        index += len(funs)
        dll_name = get_dllname(x)
        res[dll_name] = funs
    return res

def out_file(contents, path):
    full_path = os.path.join(args.bin_dir, path)
    dir_path = os.path.dirname(full_path)
    os.makedirs(dir_path, exist_ok=True)
    with open(full_path, 'w') as f:
        f.write(contents)

args = parser.parse_args()

if args.print_out_files:
    res = []

    for x in args.filenames:
        if not os.path.exists(x):
            raise RuntimeError("Nonexistent input file passed: " + x)
        dllname = get_dllname(x)
        res.append(INTERFACE_CPP_OUT_FILE_TEMPLATE % dllname)
        res.append(INTERFACE_H_OUT_FILE_TEMPLATE % dllname)

    res.append(DLL_CTX_H_OUT_FILE)
    res.append(DISPATCHER_CPP_OUT_FILE)

    print(';'.join(os.path.join(args.bin_dir, x) for x in res))
else:
    os.makedirs(args.bin_dir, exist_ok=True)
    for x in args.filenames:
        if not os.path.exists(x):
            raise RuntimeError("Nonexistent input file passed: " + x)
    dlls = parse_dlls(args.filenames)

    for k in dlls:
        dllname = get_dllname(k)
        dll = dlls[k]
        h, cpp = render_dll(dllname, [ x for i, x in dll ])
        out_file(cpp, INTERFACE_CPP_OUT_FILE_TEMPLATE % dllname)
        out_file(h, INTERFACE_H_OUT_FILE_TEMPLATE % dllname)

    out_file(dll_ctx_h_template.render(dll_names=list(get_dllname(x) for x in args.filenames)),
        DLL_CTX_H_OUT_FILE)

    funs = []
    for k in dlls:
        dll = dlls[k]
        for i, fun in dll:
            funs.append((i, k, fun.name))

    out_file(dispatcher_cpp_template.render(funs=funs), DISPATCHER_CPP_OUT_FILE)
