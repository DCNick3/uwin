
#include <stdio.h>
#include <stdlib.h>
#include <windows.h>

int PASCAL WinMain( HINSTANCE hInstance, HINSTANCE hPrevInstance,
                        LPSTR lpCmdLine, int nCmdShow)
{
    CHAR szBuffer[1000];

    if (GetModuleFileNameA(NULL, szBuffer, sizeof(szBuffer) / sizeof(*szBuffer)) == 0) {
        return 1;
    }

    switch (szBuffer[0] % 7) {
        case 0: MessageBoxA(0, "case 0", "message", 0);       break;
        case 1: MessageBoxA(0, "case 1", "message", 0);       break;
        case 2: MessageBoxA(0, "case 2", "message", 0);       break;
        case 3: MessageBoxA(0, "case 3", "message", 0);       break;
        case 4: MessageBoxA(0, "case 4", "message", 0);       break;
        case 5: MessageBoxA(0, "case 5", "message", 0);       break;
        case 6: MessageBoxA(0, "case 6", "message", 0);       break;
        default:  MessageBoxA(0, "default case", "message", 0); break;
    }

    return 0;
}
