from c_types import resolve_decl, CTYPECONV

from pycparser.c_ast import ID

from typing import List


class ARG:
    argtype: CTYPECONV
    argname: str

    def __init__(self, decl):
        self.argtype, self.argname = resolve_decl(decl.type)

    def __repr__(self) -> str:
        return '%s %s' % (self.argtype.uwtype, self.argname)


CALLING_CONVENTIONS = {'stdcall'}


class FUN:
    ret_type: CTYPECONV
    name: str
    calling_convention: str
    args: List[ARG]

    def __init__(self, decl, funcspec):
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

        if decl.args != None:
            for arg in decl.args:
                self.args.append(ARG(arg))

    def proto(self, namespace) -> str:
        return '%s %s(%s)' % (
            'void' if self.ret_type is None else self.ret_type.uwtype,
            ('' if namespace is None else namespace + "::") + self.name, ', '.join(map(repr, self.args)))
    def __repr__(self) -> str:
        return self.proto(None)
