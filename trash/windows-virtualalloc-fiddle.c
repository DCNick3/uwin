
#include <windows.h>
#include <tchar.h>
#include <stdio.h>
#include <stdlib.h>             // For exit

#define PAGELIMIT 80            // Number of pages to ask for

LPTSTR lpNxtPage;               // Address of the next page to ask for
DWORD dwPages = 0;              // Count of pages gotten so far
DWORD dwPageSize;               // Page size on this computer

VOID __declspec(noreturn) ErrorExit(LPTSTR lpMsg)
{
    _tprintf(TEXT("Error! %s with error code of %ld.\n"),
        lpMsg, GetLastError());
    exit(0);
}

INT _tmain(VOID)
{
    LPVOID lpvBase;               // Base address of the test memory
    LPTSTR lpPtr;                 // Generic character pointer
    BOOL bSuccess;                // Flag
    DWORD i;                      // Generic counter
    SYSTEM_INFO sSysInfo;         // Useful information about the system

    GetSystemInfo(&sSysInfo);     // Initialize the structure.

    _tprintf(TEXT("This computer has page size %d.\n"), sSysInfo.dwPageSize);

    dwPageSize = 0x1000;

    // Reserve pages in the virtual address space of the process.

    lpvBase = VirtualAlloc(
        NULL,                 // System selects address
        PAGELIMIT * dwPageSize, // Size of allocation
        MEM_RESERVE,          // Allocate reserved pages
        PAGE_EXECUTE_READ);       // Protection = no access
    if (lpvBase == NULL)
        ErrorExit(TEXT("VirtualAlloc reserve failed."));

    MEMORY_BASIC_INFORMATION mbi;
    if (VirtualQuery(lpvBase, &mbi, sizeof(mbi)) == 0)
        ErrorExit(TEXT("VirtualQuery failed"));
    _tprintf(TEXT("Protect = 0x%08x\n"), mbi.Protect);

    _tprintf(TEXT("First commit\n"));
    if (VirtualAlloc(lpvBase, 0x1000, MEM_COMMIT, PAGE_READWRITE) == NULL)
        ErrorExit(TEXT("First commit failed"));

    if (VirtualQuery(lpvBase, &mbi, sizeof(mbi)) == 0)
        ErrorExit(TEXT("VirtualQuery failed"));
    _tprintf(TEXT("Protect = 0x%08x\n"), mbi.Protect);

    memset(lpvBase, 12, 0x1000);

    _tprintf(TEXT("Second commit\n"));
    if (VirtualAlloc(lpvBase, 0x2000, MEM_COMMIT, PAGE_READONLY) == NULL)
        ErrorExit(TEXT("Second commit failed"));

    if (VirtualQuery(lpvBase, &mbi, sizeof(mbi)) == 0)
        ErrorExit(TEXT("VirtualQuery failed"));
    _tprintf(TEXT("Protect = 0x%08x\n"), mbi.Protect);

    if (*((BYTE*)lpvBase) == 12) {
        _tprintf(TEXT("The value was kept the same\n"));
    }
    else {
        _tprintf(TEXT("The value has changed\n"));
    }

    _tprintf(TEXT("Decommit\n"));
    if (!VirtualFree(lpvBase, 0x1000, MEM_DECOMMIT))
        ErrorExit(TEXT("Decommit failed"));

    if (VirtualQuery(lpvBase, &mbi, sizeof(mbi)) == 0)
        ErrorExit(TEXT("VirtualQuery failed"));
    _tprintf(TEXT("Protect = 0x%08x\n"), mbi.Protect);

    // Release the block of pages when you are finished using them.

    bSuccess = VirtualFree(
        lpvBase,       // Base address of block
        0,             // Bytes of committed pages
        MEM_RELEASE);  // Decommit the pages

    _tprintf(TEXT("Release %s.\n"), bSuccess ? TEXT("succeeded") : TEXT("failed"));

    return 0;
}
