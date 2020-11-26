
from pycparser.c_ast import PtrDecl, TypeDecl, IdentifierType

from typing import Tuple, Optional

# this design will support only little endian..
# supporting big endian is quite ambitious, but not too useful
typetable = dict()

RAWTYPES = {'s32', 'u32'}
CTOUW = {'constr'}
UWTOC = {'constr', 'value'}

class CTYPECONV:
    def __init__(self, uwtype, rawtype, ctouw, uwtoc):
        assert rawtype in RAWTYPES
        assert ctouw in CTOUW
        assert uwtoc in UWTOC
        self.uwtoc = uwtoc
        self.ctouw = ctouw
        self.rawtype = rawtype
        self.uwtype = uwtype

    def __repr__(self):
        return '(%s, %s, %s, %s)' % (self.uwtype, self.rawtype, self.ctouw, self.uwtoc)

def add(ctype, uwtype, rawtype, ctouw='constr', uwtoc='constr'):
    if ctype in typetable:
        raise RuntimeError('Duplicate ctype')
    typetable[ctype] = CTYPECONV(uwtype, rawtype, ctouw, uwtoc)

def alias(ctype, ref):
    if ctype in typetable:
        raise RuntimeError('Duplicate ctype')
    typetable[ctype] = ref

def ptr(ctype, uwtype):
    add(ctype + ' *', 'uwin::mem::tptr<%s>' % uwtype, 'u32', 'constr', 'constr')
    add(ctype + ' const *', 'uwin::mem::tcptr<%s>' % uwtype, 'u32', 'constr', 'constr')

def hndl(ctype, uwobj):
    add(ctype, 'uwin::win32::types::handle<%s>' % uwobj, 'u32', 'constr', 'constr')

def winstruct(ctype, uwtype):
    ptr(ctype, uwtype)
    alias('P' + ctype, ctype + ' *')
    alias('LP' + ctype, ctype + ' *')

"""
========================================================================================================================
"""

add('unsigned int', 'std::uint32_t', 'u32')
add('signed int', 'std::int32_t', 'u32')
add('_Bool', 'bool', 'u32')

alias('int', 'signed int')
alias('signed', 'signed int')
alias('unsigned', 'unsigned int')
alias('BOOL', '_Bool')

alias('INT', 'int')
alias('UINT', 'unsigned int')
alias('DWORD', 'UINT')

hndl('HWND', 'uwin::win32::types::wnd')

ptr('char', 'char')
alias('LPSTR', 'char *')
alias('LPCSTR', 'char const *')

winstruct('FILETIME', 'uwin::win32::types::FILETIME')

"""
========================================================================================================================
"""


for x in range(100):
    t = True
    for key in typetable:
        value = typetable[key]
        if type(value) is str:
            typetable[key] = typetable[value]
    if t:
        break
else:
    raise RuntimeError("Cannot resolve type aliases. Is there a loop?")

#pprint(typetable)

def resolve_decl_intrnl(decl):
    if type(decl) is PtrDecl:
        res, nm = resolve_decl_intrnl(decl.type)
        res += ' *'
        for x in decl.quals:
            res += ' ' + x
        return res, nm
    elif type(decl) is TypeDecl:
        assert type(decl.type) is IdentifierType
        res = ' '.join(decl.type.names)
        for x in decl.quals:
            res += ' ' + x
        return res, decl.declname

    decl.show()
    raise RuntimeError('Unsupported declaration')
    #return "???", "???"

def resolve_decl(decl) -> Tuple[Optional[CTYPECONV], str]:
    type, name = resolve_decl_intrnl(decl)
    if type == 'void':
        return None, name
    return typetable[type], name