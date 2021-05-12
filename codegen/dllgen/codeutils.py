
from model import FUN
from c_types import CTYPECONV


def stdcall_arg(type: CTYPECONV, varname: str, index: int):
    if type.ctouw == 'constr':
        return '%s %s(get_stdcall_%s(st, %d));\n' % (type.uwtype, varname, type.rawtype, index)
    else:
        raise NotImplementedError()

def stdcall_result(type: CTYPECONV, varname: str):
    if type.uwtoc == 'constr':
        return 'stdcall_set_result_%s(st, %s);\n' % (type.rawtype, varname)
    elif type.uwtoc == 'value':
        return 'stdcall_set_result_%s(st, %s.value());\n' % (type.rawtype, varname)
    else:
        raise NotImplementedError()


def cdecl_arg(type: CTYPECONV, varname: str, index: int):
    if type.ctouw == 'constr':
        return '%s %s(get_cdecl_%s(st, %d));\n' % (type.uwtype, varname, type.rawtype, index)
    else:
        raise NotImplementedError()

def cdecl_result(type: CTYPECONV, varname: str):
    if type.uwtoc == 'constr':
        return 'cdecl_set_result_%s(st, %s);\n' % (type.rawtype, varname)
    elif type.uwtoc == 'value':
        return 'cdecl_set_result_%s(st, %s.value());\n' % (type.rawtype, varname)
    else:
        raise NotImplementedError()

def gen_remill_entry(fun: FUN):
    res = ''
    pad = '        '
    if fun.calling_convention == 'stdcall':
        assert not fun.vararg
        for i, arg in enumerate(fun.args):
            res += pad + stdcall_arg(arg.argtype, arg.argname, i)
        res += '\n'
        call_str = f'{fun.name}({", ".join(f"std::move({x.argname})" for x in fun.args)})'
        if fun.ret_type is None:
            res += pad + f'{call_str};\n'
        else:
            res += pad + f'{fun.ret_type.uwtype} result = {call_str};\n'
            res += pad + stdcall_result(fun.ret_type, 'result')
        res += '\n'
        res += pad + 'stdcall_ret(st, %d);\n' % len(fun.args)
        return res
    elif fun.calling_convention == 'cdecl':
        for i, arg in enumerate(fun.args):
            res += pad + cdecl_arg(arg.argtype, arg.argname, i)
        res += '\n'
        call_str = f'{fun.name}({", ".join(f"std::move({x.argname})" for x in fun.args)}' + \
            f'{"" if not fun.vararg else ", cdecl_get_vararg_ctx(st, " + str(len(fun.args)) + ")"})'
        if fun.ret_type is None:
            res += pad + f'{call_str};\n'
        else:
            res += pad + f'{fun.ret_type.uwtype} result = {call_str};\n'
            res += pad + cdecl_result(fun.ret_type, 'result')
        res += '\n'
        res += pad + 'cdecl_ret(st, %d);\n' % len(fun.args)
        return res
    else:
        raise RuntimeError("Unknown calling convention: " + fun.calling_convention)
