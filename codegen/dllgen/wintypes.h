
#pragma once

// actually those are dummy definitions; the type mapping is hard-coded into the parser

typedef void* HWND;
typedef const char* LPCSTR;
typedef unsigned int UINT;
typedef int BOOL;
typedef UINT DWORD;

typedef struct FILETIME FILETIME, *LPFILETIME;

#define WINAPI __attribute__((stdcall))