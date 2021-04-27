
#pragma once

// actually those are dummy definitions; the type mapping is done in c_types.py
// those are only needed in order for parser to work

#define WINAPI __attribute__((stdcall))
#define CDECL __attribute__((cdecl))
#define WINAPIV CDECL
#define WINBASEAPI
#define WINUSERAPI
#define NTSYSAPI
#define FAR
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
typedef void *HCURSOR, *HINSTANCE, *HICON, *HMENU;
typedef short ATOM;
typedef signed long LRESULT;
typedef unsigned int WPARAM;
typedef signed long LPARAM;
typedef signed long HRESULT;

typedef struct FILETIME FILETIME, *LPFILETIME;

typedef struct _EXCEPTION_POINTERS _EXCEPTION_POINTERS;
typedef struct EXCEPTION_RECORD EXCEPTION_RECORD, *PEXCEPTION_RECORD;
typedef struct CPINFO CPINFO, *LPCPINFO;
typedef struct WNDCLASSA WNDCLASSA;
typedef struct MSG MSG, *LPMSG;
typedef struct GUID GUID;
typedef struct DIRECTDRAW DIRECTDRAW, *LPDIRECTDRAW;
typedef struct IUnknown IUnknown;
typedef struct SECURITY_ATTRIBUTES *LPSECURITY_ATTRIBUTES;
typedef struct MEMORY_BASIC_INFORMATION *PMEMORY_BASIC_INFORMATION;

typedef
BOOL
// TODO: implement attributes at function pointer definitions
(/*WINAPI*/ *PHANDLER_ROUTINE)(
DWORD CtrlType
);
typedef LONG (/*WINAPI*/ *PTOP_LEVEL_EXCEPTION_FILTER)(
    struct _EXCEPTION_POINTERS *ExceptionInfo
    );
typedef PTOP_LEVEL_EXCEPTION_FILTER LPTOP_LEVEL_EXCEPTION_FILTER;
