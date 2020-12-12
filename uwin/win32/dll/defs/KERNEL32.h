WINBASEAPI
HANDLE
WINAPI
HeapCreate(
    DWORD flOptions,
    DWORD dwInitialSize,
    DWORD dwMaximumSize
    );

WINBASEAPI
BOOL
WINAPI
HeapDestroy(
    HANDLE hHeap
    );


WINBASEAPI
LPVOID
WINAPI
HeapAlloc(
    HANDLE hHeap,
    DWORD dwFlags,
    DWORD dwBytes
    );

WINBASEAPI
LPVOID
WINAPI
HeapReAlloc(
    HANDLE hHeap,
    DWORD dwFlags,
    LPVOID lpMem,
    DWORD dwBytes
    );

WINBASEAPI
BOOL
WINAPI
HeapFree(
    HANDLE hHeap,
    DWORD dwFlags,
    LPVOID lpMem
    );

WINBASEAPI
DWORD
WINAPI
HeapSize(
    HANDLE hHeap,
    DWORD dwFlags,
    LPCVOID lpMem
    );

WINBASEAPI
BOOL
WINAPI
HeapValidate(
    HANDLE hHeap,
    DWORD dwFlags,
    LPCVOID lpMem
    );

WINBASEAPI
BOOL
WINAPI
CloseHandle(
    HANDLE hObject
    );

WINBASEAPI
HMODULE
WINAPI
GetModuleHandleA(
    LPCSTR lpModuleName
    );

WINBASEAPI
VOID
WINAPI
GetStartupInfoA(
    LPSTARTUPINFOA lpStartupInfo
    );

WINBASEAPI
LPSTR
WINAPI
GetCommandLineA(
    VOID
    );

WINBASEAPI
DWORD
WINAPI
GetVersion( VOID );

WINBASEAPI
VOID
WINAPI
ExitProcess(
    UINT uExitCode
    );

WINBASEAPI
BOOL
WINAPI
TerminateProcess(
    HANDLE hProcess,
    UINT uExitCode
    );

WINBASEAPI
HANDLE
WINAPI
GetCurrentProcess(
    VOID
    );

WINBASEAPI
LONG
WINAPI
UnhandledExceptionFilter(
    _EXCEPTION_POINTERS *ExceptionInfo
    );

WINBASEAPI
DWORD
WINAPI
GetModuleFileNameA(
    HMODULE hModule,
    LPSTR lpFilename,
    DWORD nSize
    );

WINBASEAPI
BOOL
WINAPI
FreeEnvironmentStringsA(
    LPSTR
    );
WINBASEAPI
BOOL
WINAPI
FreeEnvironmentStringsW(
    LPWSTR
    );

WINBASEAPI
LPSTR
WINAPI
GetEnvironmentStrings(
    VOID
    );

WINBASEAPI
LPWSTR
WINAPI
GetEnvironmentStringsW(
    VOID
    );

WINBASEAPI
UINT
WINAPI
SetHandleCount(
    UINT uNumber
    );

WINBASEAPI
HANDLE
WINAPI
GetStdHandle(
    DWORD nStdHandle
    );

WINBASEAPI
DWORD
WINAPI
GetFileType(
    HANDLE hFile
    );

WINBASEAPI
LPVOID
WINAPI
VirtualAlloc(
    LPVOID lpAddress,
    DWORD dwSize,
    DWORD flAllocationType,
    DWORD flProtect
    );

WINBASEAPI
BOOL
WINAPI
VirtualFree(
    LPVOID lpAddress,
    DWORD dwSize,
    DWORD dwFreeType
    );

WINBASEAPI
BOOL
WINAPI
VirtualProtect(
    LPVOID lpAddress,
    DWORD dwSize,
    DWORD flNewProtect,
    PDWORD lpflOldProtect
    );

WINBASEAPI
BOOL
WINAPI
WriteFile(
    HANDLE hFile,
    LPCVOID lpBuffer,
    DWORD nNumberOfBytesToWrite,
    LPDWORD lpNumberOfBytesWritten,
    LPOVERLAPPED lpOverlapped
    );

WINBASEAPI
DWORD
WINAPI
GetLastError(
    VOID
    );

WINBASEAPI
DWORD
WINAPI
SetFilePointer(
    HANDLE hFile,
    LONG lDistanceToMove,
    PLONG lpDistanceToMoveHigh,
    DWORD dwMoveMethod
    );

WINBASEAPI
FARPROC
WINAPI
GetProcAddress(
    HMODULE hModule,
    LPCSTR lpProcName
    );

WINBASEAPI
HMODULE
WINAPI
LoadLibraryA(
    LPCSTR lpLibFileName
    );

WINBASEAPI
BOOL
WINAPI
SetStdHandle(
    DWORD nStdHandle,
    HANDLE hHandle
    );

WINBASEAPI
BOOL
WINAPI
FlushFileBuffers(
    HANDLE hFile
    );

WINBASEAPI
int
WINAPI
MultiByteToWideChar(
    UINT     CodePage,
    DWORD    dwFlags,
    LPCSTR   lpMultiByteStr,
    int      cchMultiByte,
    LPWSTR   lpWideCharStr,
    int      cchWideChar);

WINBASEAPI
int
WINAPI
WideCharToMultiByte(
    UINT     CodePage,
    DWORD    dwFlags,
    LPCWSTR  lpWideCharStr,
    int      cchWideChar,
    LPSTR    lpMultiByteStr,
    int      cchMultiByte,
    LPCSTR   lpDefaultChar,
    LPBOOL   lpUsedDefaultChar);

NTSYSAPI WINAPI VOID RtlUnwind(
  PVOID             TargetFrame,
  PVOID             TargetIp,
  PEXCEPTION_RECORD ExceptionRecord,
  PVOID             ReturnValue
);

WINBASEAPI
UINT
WINAPI
GetACP(void);

WINBASEAPI
UINT
WINAPI
GetOEMCP(void);

WINBASEAPI
BOOL
WINAPI
GetCPInfo(
    UINT      CodePage,
    LPCPINFO  lpCPInfo);

WINBASEAPI
int
WINAPI
LCMapStringA(
    LCID     Locale,
    DWORD    dwMapFlags,
    LPCSTR lpSrcStr,
    int      cchSrc,
    LPSTR  lpDestStr,
    int      cchDest);

WINBASEAPI
int
WINAPI
LCMapStringW(
    LCID     Locale,
    DWORD    dwMapFlags,
    LPCWSTR lpSrcStr,
    int      cchSrc,
    LPWSTR  lpDestStr,
    int      cchDest);

WINBASEAPI
BOOL
WINAPI
GetStringTypeA(
    LCID     Locale,
    DWORD    dwInfoType,
    LPCSTR   lpSrcStr,
    int      cchSrc,
    LPWORD   lpCharType);

WINBASEAPI
BOOL
WINAPI
GetStringTypeW(
    DWORD    dwInfoType,
    LPCWSTR  lpSrcStr,
    int      cchSrc,
    LPWORD   lpCharType);

WINBASEAPI
VOID
WINAPI
DebugBreak(
    VOID
    );

LONG
WINAPI
InterlockedIncrement(
    LPLONG lpAddend
    );

LONG
WINAPI
InterlockedDecrement(
    LPLONG lpAddend
    );

WINBASEAPI
VOID
WINAPI
OutputDebugStringA(
    LPCSTR lpOutputString
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleCtrlHandler(
    PHANDLER_ROUTINE HandlerRoutine,
    BOOL Add
    );

WINBASEAPI
BOOL
WINAPI
IsBadReadPtr(
    CONST VOID *lp,
    UINT ucb
    );

WINBASEAPI
BOOL
WINAPI
IsBadWritePtr(
    LPVOID lp,
    UINT ucb
    );