/*UW_BUILD:
{
    "link_deps": ["simple_dll"]
}
:UW_BUILD*/

#include <stdio.h>
#include <windows.h>

__declspec(dllimport)
INT WINAPI GetProcAttachCounter(VOID);

int main() {
    printf("AttachCounter = %d\n", GetProcAttachCounter());

    return 0;
}