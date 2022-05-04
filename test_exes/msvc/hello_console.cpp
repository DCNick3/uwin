/*UW_BUILD:
{}
:UW_BUILD*/

#include <stdio.h>
#include <stdlib.h>
#include <windows.h>

int main()
{
    char buffer[1024];

    printf("Hello, console!\n");

    printf("What's your name? ");

    gets(buffer);

    printf("Hello you too, %s\n", buffer);

    return 0;
}
