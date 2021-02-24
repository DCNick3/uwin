from c_types import resolve_decl, CTYPECONV

from pycparser.c_ast import ID, EllipsisParam

from typing import List


class ARG:
    argtype: CTYPECONV
    argname: str

    def __init__(self, decl, num):
        self.argtype, self.argname = resolve_decl(decl.type)
        if self.argname is None:
            self.argname = "arg%d" % num

    def __repr__(self) -> str:
        return '%s %s' % (self.argtype.uwtype, self.argname)


CALLING_CONVENTIONS = {'stdcall', 'cdecl'}


class FUN:
    ret_type: CTYPECONV
    name: str
    calling_convention: str
    args: List[ARG]
    vararg: bool

    def __init__(self, decl, funcspec):
        self.vararg = False

        self.ret_type, self.name = resolve_decl(decl.type)

        self.calling_convention = '???'

        for x in funcspec:
            exprs = x.exprlist.exprs
            assert len(exprs) == 1
            assert type(exprs[0]) is ID
            id = exprs[0].name
            assert id in CALLING_CONVENTIONS
            self.calling_convention = id

        self.args = []

        if decl.args is not None:
            for i, arg in enumerate(decl.args):
                if type(arg) is EllipsisParam:
                    self.vararg = True
                else:
                    arg = ARG(arg, i)
                    if arg.argtype is not None:
                        self.args.append(arg)

    def proto(self, namespace) -> str:
        return '%s %s(%s%s)' % (
            'void' if self.ret_type is None else self.ret_type.uwtype,
            ('' if namespace is None else namespace + "::") + self.name, ', '.join(map(repr, self.args)),
            ('' if not self.vararg else ", uwin::win32::dll::vararg_ctx varargs"))
    def __repr__(self) -> str:
        return self.proto(None)
