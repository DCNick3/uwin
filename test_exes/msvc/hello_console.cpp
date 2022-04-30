
#include <stdio.h>
#include <stdlib.h>
#include <windows.h>

int PASCAL WinMain( HINSTANCE hInstance, HINSTANCE hPrevInstance,
                        LPSTR lpCmdLine, int nCmdShow)
{
    char buffer[1024];

    printf("Hello, console!\n");

    printf("What's your name? ");

    gets(buffer);

    printf("Hello you too, %s\n", buffer);

    return 0;
}
