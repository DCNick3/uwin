
from pycparser.c_ast import PtrDecl, TypeDecl, IdentifierType

from typing import Tuple, Optional

# this design will support only little endian..
# supporting big endian is quite ambitious, but not too useful
typetable = dict()

RAWTYPES = {'s16', 'u16', 's32', 'u32', 'void'}
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
    add(ctype + ' *', 'uwin::mem::tptr<%s>' % uwtype, 'u32', 'constr', 'value')
    add(ctype + ' const *', 'uwin::mem::tcptr<%s>' % uwtype, 'u32', 'constr', 'value')

def hndl(ctype, uwobj):
    add(ctype, 'uwin::ht::handle<%s>' % uwobj, 'u32', 'constr', 'value')

def winstruct(ctype, uwtype):
    ptr(ctype, uwtype)
    alias('P' + ctype, ctype + ' *')
    alias('LP' + ctype, ctype + ' *')

"""
========================================================================================================================
"""

add('unsigned int', 'std::uint32_t', 'u32')
add('signed int', 'std::int32_t', 'u32')
add('unsigned long', 'std::uint32_t', 'u32')
add('signed long', 'std::int32_t', 'u32')
add('wchar_t', 'std::uint16_t', 'u16')
add('unsigned short', 'std::uint16_t', 'u16')
add('signed short', 'std::int16_t', 's16')
add('_Bool', 'bool', 'u32')
add('void', 'void', 'void')

alias('int', 'signed int')
alias('signed', 'signed int')
alias('unsigned', 'unsigned int')
alias('short', 'signed short')
alias('long', 'signed long')

alias('BOOL', '_Bool')
alias('INT', 'int')
alias('LONG', 'long')
alias('UINT', 'unsigned int')
alias('USHORT', 'unsigned short')
alias('ULONG', 'unsigned long')
alias('WORD', 'USHORT')
alias('DWORD', 'UINT')

hndl('HWND', 'uwin::win32::types::wnd')
hndl('HANDLE', 'uwin::ht::kobj')

add('HMODULE', 'uwin::win32::types::hmodule', 'u32')

ptr('char', 'char')
ptr('wchar_t', 'wchar_t')
ptr('void', 'void')
ptr('WORD', 'uint16_t')
ptr('DWORD', 'uint32_t')
ptr('LONG', 'int32_t')
ptr('_Bool', 'uwin::win32::types::BOOL')

alias('LPSTR', 'char *')
alias('LPCSTR', 'char const *')
alias('LPWSTR', 'wchar_t *')
alias('LPCWSTR', 'wchar_t const *')
alias('PVOID', 'void *')
alias('LPVOID', 'void *')
alias('LPCVOID', 'void const *')
alias('PDWORD', 'DWORD *')
alias('LPDWORD', 'DWORD *')
alias('PLONG', 'LONG *')
alias('LPLONG', 'LONG *')
alias('FARPROC', 'LPVOID')
alias('PBOOL', '_Bool *')
alias('LPBOOL', '_Bool *')
alias('LCID', 'DWORD')
alias('LPWORD', 'WORD *')
alias('PHANDLER_ROUTINE', 'void *') # TODO: implement function pointers

winstruct('FILETIME', 'uwin::win32::types::FILETIME')
winstruct('STARTUPINFOA', 'uwin::win32::types::STARTUPINFOA')
winstruct('_EXCEPTION_POINTERS', 'uwin::win32::types::EXCEPTION_POINTERS')
winstruct('OVERLAPPED', 'uwin::win32::types::OVERLAPPED')
winstruct('EXCEPTION_RECORD', 'uwin::win32::types::EXCEPTION_RECORD')
winstruct('CPINFO', 'uwin::win32::types::CPINFO')

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
    type = typetable[type]
    if type.rawtype == 'void':
        type = None
    return type, name