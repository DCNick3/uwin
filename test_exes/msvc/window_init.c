/*UW_BUILD:
{}
:UW_BUILD*/

#define NAME "Window init"

#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <windowsx.h>
#include <stdlib.h>
#include <stdarg.h>

#define TIMER_ID        1
#define TIMER_RATE      500

BOOL                    bActive;        // is application active?

long FAR PASCAL WindowProc( HWND hWnd, UINT message,
                            WPARAM wParam, LPARAM lParam )
{
    PAINTSTRUCT ps;
    RECT        rc;
    SIZE        size;
    static BYTE phase = 0;

    switch( message )
    {
        case WM_ACTIVATEAPP:
            printf("WM_ACTIVATEAPP\n");
            bActive = wParam;
            break;

        case WM_CREATE:
            printf("WM_CREATE\n");
            break;

        case WM_MOUSEMOVE:
            printf("WM_MOUSEMOVE(x=%5d, y=%5d)\n", GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam));
            break;

        case WM_KEYDOWN:
            printf("WM_KEYDOWN(0x%08x)\n", wParam);
            switch( wParam )
            {
                case VK_ESCAPE:
                case VK_F12:
                    PostMessage(hWnd, WM_CLOSE, 0, 0);
                    break;
            }
            break;

        case WM_DESTROY:
            printf("WM_DESTROY\n");
            PostQuitMessage( 0 );
            break;
    }

    return DefWindowProc(hWnd, message, wParam, lParam);

} /* WindowProc */

/*
 * doInit - do work required for every instance of the application:
 *                create the window, initialize data
 */
static BOOL doInit( HINSTANCE hInstance, int nCmdShow )
{
    HWND                hwnd;
    WNDCLASS            wc;
    char                buf[256];

    /*
     * set up and register window class
     */
    wc.style = 0; // CS_HREDRAW | CS_VREDRAW;
    wc.lpfnWndProc = WindowProc;
    wc.cbClsExtra = 0;
    wc.cbWndExtra = 0;
    wc.hInstance = hInstance;
    wc.hIcon = LoadIcon( hInstance, IDI_APPLICATION );
    wc.hCursor = LoadCursor( NULL, IDC_ARROW );
    wc.hbrBackground = NULL;
    wc.lpszMenuName = NAME;
    wc.lpszClassName = NAME;
    RegisterClass( &wc );

    /*
     * create a window
     */
    hwnd = CreateWindowEx(
            0,
            NAME,
            "Just a random window",
            WS_OVERLAPPEDWINDOW,
            0, 0,
            200, 200,
            NULL,
            NULL,
            hInstance,
            NULL );

    if( !hwnd )
    {
        return FALSE;
    }

    ShowWindow( hwnd, nCmdShow );
    UpdateWindow( hwnd );

    return TRUE;
} /* doInit */

/*
 * WinMain - initialization, message loop
 */
int PASCAL WinMain( HINSTANCE hInstance, HINSTANCE hPrevInstance,
        LPSTR lpCmdLine, int nCmdShow)
{
    MSG         msg;

    lpCmdLine = lpCmdLine;
    hPrevInstance = hPrevInstance;

    if( !doInit( hInstance, nCmdShow ) )
    {
        return FALSE;
    }

    while (GetMessage(&msg, NULL, 0, 0))
    {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    return msg.wParam;
} /* WinMain */
