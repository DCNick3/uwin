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
#include <comdef.h>

#define TIMER_ID        1
#define TIMER_RATE      500

#define WIDTH 800
#define HEIGHT 600

LPDIRECTDRAW            lpDD = NULL;           // DirectDraw object
LPDIRECTDRAWSURFACE     lpDDSPrimary = NULL;   // DirectDraw primary surface
LPDIRECTDRAWSURFACE     lpDDSBack = NULL;      // DirectDraw back surface
INT                     clickCounter = 0;      // Click Counter
RECT                    rect;


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

WORD colors[] = {0xfe30, 0x5630, 0x52b9};

VOID UpdateScreen(VOID) {
    DWORD         i, j;
    DDSURFACEDESC desc;
    HRESULT       ddrval;
    char          buf[256];
    WORD          color;
    DDBLTFX       fx;

    memset(&fx, 0, sizeof(fx));
    fx.dwSize = sizeof(fx);

    memset(&desc, 0, sizeof(desc));
    desc.dwSize = sizeof(desc);

    color = colors[clickCounter % 3];

    ddrval = lpDDSBack->Lock(/* rect */NULL, &desc, DDLOCK_WAIT, /* hEvent */NULL);
    if (ddrval == DD_OK)
    {
        LONG pitch = desc.lPitch;
        WORD* buffer = (WORD*)desc.lpSurface;

        for (j = 0; j < desc.dwHeight; j++)
        for (i = 0; i < desc.dwWidth; i++)
            buffer[j * pitch / sizeof(WORD) + i] = color;

        lpDDSBack->Unlock(desc.lpSurface);
        ddrval = lpDDSPrimary->Blt(&rect, lpDDSBack, &rect, 0, &fx);
        if (ddrval == DD_OK)
        {
            return;
        }
    }


//    _com_error err(ddrval);
    wsprintf( buf, "Direct Draw Error during UpdateScreen (%08lx)\n", ddrval/*, err.ErrorMessage()*/ );
    MessageBox( 0, buf, "ERROR", MB_OK );
}

long FAR PASCAL WindowProc( HWND hWnd, UINT message,
                            WPARAM wParam, LPARAM lParam )
{
    PAINTSTRUCT ps;
    RECT        rc;
    SIZE        size;
    static BYTE phase = 0;

    switch( message )
    {
        case WM_CREATE:
            break;

        case WM_SETCURSOR:
            SetCursor(NULL);
            return TRUE;

        case WM_LBUTTONDOWN:
            UpdateScreen();
            clickCounter++;
            break;

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
                                            DDSCL_ALLOWREBOOT | DDSCL_EXCLUSIVE | DDSCL_FULLSCREEN );
        if(ddrval == DD_OK )
        {
            ddrval = lpDD->SetDisplayMode( WIDTH, HEIGHT, 16 );
            if( ddrval == DD_OK )
            {
                // Create the primary surface with 1 back buffer
                ddsd.dwSize = sizeof( ddsd );
                ddsd.dwFlags = DDSD_CAPS;
                ddsd.ddsCaps.dwCaps = DDSCAPS_PRIMARYSURFACE;
                ddrval = lpDD->CreateSurface( &ddsd, &lpDDSPrimary, NULL );
                if( ddrval == DD_OK )
                {
                    ddsd.dwFlags = DDSD_CAPS | DDSD_WIDTH | DDSD_HEIGHT;
                    ddsd.ddsCaps.dwCaps = DDSCAPS_SYSTEMMEMORY | DDSCAPS_OFFSCREENPLAIN;
                    ddsd.dwHeight = HEIGHT;
                    ddsd.dwWidth = WIDTH;
                    ddrval = lpDD->CreateSurface( &ddsd, &lpDDSBack, NULL );
                    if( ddrval == DD_OK )
                    {
                        rect.left = 0;
                        rect.top = 0;
                        rect.right = WIDTH;
                        rect.bottom = HEIGHT;

                        // draw smth before the first click
                        UpdateScreen();
                        clickCounter++;

                        return TRUE;
                    }
                }
            }
        }
    }

    //_com_error err(ddrval);
    wsprintf( buf, "Direct Draw Init Error (%08lx)\n", ddrval/*, err.ErrorMessage()*/ );
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
