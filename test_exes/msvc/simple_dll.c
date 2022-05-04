/*UW_BUILD:
{
    "kind": "Dll"
}
:UW_BUILD*/

#include <windows.h>

static INT proc_attach_counter = 0;

BOOL WINAPI DllMain (
        HINSTANCE const instance,  // handle to DLL module
        DWORD     const reason,    // reason for calling function
        LPVOID    const reserved)  // reserved
{
    // Perform actions based on the reason for calling.
    switch (reason)
    {
    case DLL_PROCESS_ATTACH:
        // Initialize once for each new process.
        // Return FALSE to fail DLL load.
        proc_attach_counter++;
        break;

    case DLL_THREAD_ATTACH:
        // Do thread-specific initialization.
        break;

    case DLL_THREAD_DETACH:
        // Do thread-specific cleanup.
        break;

    case DLL_PROCESS_DETACH:
        // Perform any necessary cleanup.
        proc_attach_counter--;
        break;
    }
    return TRUE;  // Successful DLL_PROCESS_ATTACH.
}

__declspec(dllexport)
INT WINAPI GetProcAttachCounter(
        VOID
    )
{
    return proc_attach_counter;
}