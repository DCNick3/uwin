/*UW_BUILD:
{}
:UW_BUILD*/

#define NAME "DDExample1"
#define TITLE "Direct Draw Example 1"

#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <windowsx.h>
#include <ddraw.h>
#include <stdlib.h>
#include <stdarg.h>

#define TIMER_ID        1
#define TIMER_RATE      500

#define WIDTH 800
#define HEIGHT 600

LPDIRECTDRAW            lpDD;           // DirectDraw object
LPDIRECTDRAWSURFACE     lpDDSPrimary;   // DirectDraw primary surface
LPDIRECTDRAWSURFACE     lpDDSBack;      // DirectDraw back surface
BOOL                    bActive;        // is application active?

/*
 * finiObjects
 *
 * finished with all objects we use; release them
 */
static void finiObjects( void )
{
    if( lpDD != NULL )
    {
        if( lpDDSPrimary != NULL )
        {
            lpDDSPrimary->Release();
            lpDDSPrimary = NULL;
        }
        lpDD->Release();
        lpDD = NULL;
    }
} /* finiObjects */

char szMsg[] = "Page Flipping Test: Press F12 to exit";
char szFrontMsg[] = "Front buffer (F12 to quit)";
char szBackMsg[] = "Back buffer (F12 to quit)";

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
            bActive = wParam;
            break;

        case WM_CREATE:
            break;

        case WM_SETCURSOR:
            SetCursor(NULL);
            return TRUE;

            /*case WM_TIMER:
                // Flip surfaces
                if( bActive )
                {
                    if (lpDDSBack->Lock(&hdc) == DD_OK)
                    {
                        SetBkColor( hdc, RGB( 0, 0, 255 ) );
                        SetTextColor( hdc, RGB( 255, 255, 0 ) );
                        if( phase )
                        {
                            TextOut( hdc, 0, 0, szFrontMsg, lstrlen(szBackMsg) );
                            phase = 0;
                        }
                        else
                        {
                            TextOut( hdc, 0, 0, szBackMsg, lstrlen(szBackMsg) );
                            phase = 1;
                        }
                        lpDDSBack->ReleaseDC(hdc);
                    }

                    while( 1 )
                    {
                        HRESULT ddrval;
                        ddrval = lpDDSPrimary->Flip( NULL, 0 );
                        if( ddrval == DD_OK )
                        {
                            break;
                        }
                        if( ddrval == DDERR_SURFACELOST )
                        {
                            ddrval = lpDDSPrimary->Restore();
                            if( ddrval != DD_OK )
                            {
                                break;
                            }
                        }
                        if( ddrval != DDERR_WASSTILLDRAWING )
                        {
                            break;
                        }
                    }
                }
                break;*/

        case WM_KEYDOWN:
            switch( wParam )
            {
                case VK_ESCAPE:
                case VK_F12:
                    PostMessage(hWnd, WM_CLOSE, 0, 0);
                    break;
            }
            break;

        case WM_DESTROY:
            finiObjects();
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
    DDSURFACEDESC       ddsd;
    DDSCAPS             ddscaps;
    HRESULT             ddrval;
    HDC                 hdc;
    char                buf[256];

    /*
     * set up and register window class
     */
    wc.style = CS_HREDRAW | CS_VREDRAW;
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
            WS_EX_TOPMOST,
            NAME,
            TITLE,
            WS_POPUP,
            0, 0,
            WIDTH,
            HEIGHT,
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

    /*
     * create the main DirectDraw object
     */
    ddrval = DirectDrawCreate( NULL, &lpDD, NULL );
    if( ddrval == DD_OK )
    {
        // Get exclusive mode
        ddrval = lpDD->SetCooperativeLevel( hwnd,
                                            DDSCL_EXCLUSIVE | DDSCL_FULLSCREEN );
        if(ddrval == DD_OK )
        {
            ddrval = lpDD->SetDisplayMode( WIDTH, HEIGHT, 16 );
            if( ddrval == DD_OK )
            {
                return TRUE;

                // Create the primary surface with 1 back buffer
                ddsd.dwSize = sizeof( ddsd );
                ddsd.dwFlags = DDSD_CAPS | DDSD_BACKBUFFERCOUNT;
                ddsd.ddsCaps.dwCaps = DDSCAPS_PRIMARYSURFACE |
                                      DDSCAPS_FLIP |
                                      DDSCAPS_COMPLEX;
                ddsd.dwBackBufferCount = 1;
                ddrval = lpDD->CreateSurface( &ddsd, &lpDDSPrimary, NULL );
                if( ddrval == DD_OK )
                {
                    // Get a pointer to the back buffer
                    ddscaps.dwCaps = DDSCAPS_BACKBUFFER;
                    ddrval = lpDDSPrimary->GetAttachedSurface(&ddscaps,
                                                              &lpDDSBack);
                    if( ddrval == DD_OK )
                    {
                        // Create a timer to flip the pages
                        if( SetTimer( hwnd, TIMER_ID, TIMER_RATE, NULL ) )
                        {
                            return TRUE;
                        }
                    }
                }
            }
        }
    }

    wsprintf(buf, "Direct Draw Init Failed (%08lx)\n", ddrval );
    MessageBox( hwnd, buf, "ERROR", MB_OK );
    finiObjects();
    DestroyWindow( hwnd );
    return FALSE;
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
