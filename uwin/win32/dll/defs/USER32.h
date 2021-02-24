int WINAPI MessageBoxA(
        HWND   hWnd,
        LPCSTR lpText,
        LPCSTR lpCaption,
        UINT   uType
);

WINUSERAPI
HCURSOR
WINAPI
LoadCursorA(
    HINSTANCE hInstance,
    LPCSTR lpCursorName);

WINUSERAPI
HICON
WINAPI
LoadIconA(
    HINSTANCE hInstance,
    LPCSTR lpIconName);

WINUSERAPI
ATOM
WINAPI
RegisterClassA(
    CONST WNDCLASSA *lpWndClass);

WINUSERAPI
int
WINAPI
GetSystemMetrics(
    int nIndex);

WINUSERAPI
HWND
WINAPI
CreateWindowExA(
    DWORD dwExStyle,
    LPCSTR lpClassName,
    LPCSTR lpWindowName,
    DWORD dwStyle,
    int X,
    int Y,
    int nWidth,
    int nHeight,
    HWND hWndParent ,
    HMENU hMenu,
    HINSTANCE hInstance,
    LPVOID lpParam);

WINUSERAPI
BOOL
WINAPI
ShowWindow(
    HWND hWnd,
    int nCmdShow);

WINUSERAPI
BOOL
WINAPI
UpdateWindow(
    HWND hWnd);

WINUSERAPI int WINAPIV wsprintfA(LPSTR, LPCSTR, ...);

WINUSERAPI
BOOL
WINAPI
DestroyWindow(
    HWND hWnd);

WINUSERAPI
BOOL
WINAPI
GetMessageA(
    LPMSG lpMsg,
    HWND hWnd ,
    UINT wMsgFilterMin,
    UINT wMsgFilterMax);

WINUSERAPI
LRESULT
WINAPI
DefWindowProcA(
    HWND hWnd,
    UINT Msg,
    WPARAM wParam,
    LPARAM lParam);

WINUSERAPI
BOOL
WINAPI
TranslateMessage(
    CONST MSG *lpMsg);

WINUSERAPI
LONG
WINAPI
DispatchMessageA(
    CONST MSG *lpMsg);

WINUSERAPI
BOOL
WINAPI
PostMessageA(
    HWND hWnd,
    UINT Msg,
    WPARAM wParam,
    LPARAM lParam);

WINUSERAPI
VOID
WINAPI
PostQuitMessage(
    int nExitCode);

WINUSERAPI
HCURSOR
WINAPI
SetCursor(
    HCURSOR hCursor);