
from model import FUN
from c_types import CTYPECONV


def stdcall_arg(type: CTYPECONV, varname: str, index: int):
    if type.ctouw == 'constr':
        return '%s %s(get_stdcall_%s(st, %d));\n' % (type.uwtype, varname, type.rawtype, index)
    else:
        raise NotImplementedError()

def stdcall_result(type: CTYPECONV, varname: str):
    if type.ctouw == 'constr':
        return 'stdcall_set_result_%s(st, %s);\n' % (type.rawtype, varname)
    elif type.ctouw == 'value':
        return 'stdcall_set_result_%s(st, %s.value);\n' % (type.rawtype, varname)
    else:
        raise NotImplementedError()

def gen_remill_entry(fun: FUN):
    res = ''
    if fun.calling_convention == 'stdcall':
        for i, arg in enumerate(fun.args):
            res += stdcall_arg(arg.argtype, arg.argname, i)
        res += '\n'
        call_str = f'{fun.name}({", ".join(f"std::move({x.argname})" for x in fun.args)})'
        if fun.ret_type is None:
            res += f'{call_str};\n'
        else:
            res += f'{fun.ret_type.uwtype} result = {call_str};\n'
            res += stdcall_result(fun.ret_type, 'result')
        res += '\n'
        res += 'stdcall_ret(st, %d);\n' % len(fun.args)
        return res
    else:
        raise RuntimeError("Unknown calling convention: " + fun.calling_convention)
