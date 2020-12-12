
#pragma once

// actually those are dummy definitions; the type mapping is hard-coded into the parser

#define WINAPI __attribute__((stdcall))
#define WINBASEAPI
#define NTSYSAPI
#define CONST const
#define VOID void

typedef void* HWND;
typedef void* HANDLE;
typedef void* LPVOID;
typedef const void* LPCVOID;
typedef const char* LPCSTR;
typedef char* LPSTR;
typedef char* LPWSTR;
typedef const char* LPCWSTR;
typedef void* HMODULE;
typedef void* LPSTARTUPINFOA;
typedef unsigned int UINT;
typedef int BOOL;
typedef UINT DWORD;
typedef long LONG, *LPLONG;
typedef DWORD *PDWORD, *LPDWORD;
typedef void *LPOVERLAPPED;
typedef LONG *PLONG;
typedef void* FARPROC;
typedef _Bool* LPBOOL;
typedef void* PVOID;
typedef DWORD LCID;
typedef unsigned short WORD, *LPWORD;

typedef struct FILETIME FILETIME, *LPFILETIME;

typedef struct _EXCEPTION_POINTERS _EXCEPTION_POINTERS;
typedef struct EXCEPTION_RECORD EXCEPTION_RECORD, *PEXCEPTION_RECORD;
typedef struct CPINFO CPINFO, *LPCPINFO;

typedef
BOOL
// TODO: implement attributes at function pointer definitions
(/*WINAPI*/ *PHANDLER_ROUTINE)(
DWORD CtrlType
);