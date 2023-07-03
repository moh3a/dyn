// https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain

#include <windows.h>

BOOL WINAPI DllMain(
    HINSTANCE hModule, // handle to DLL module
    DWORD Reason,      // reason for calling function
    LPVOID lpvReserved // reserved
)
{
    switch (Reason)
    {
    case DLL_PROCESS_ATTACH:
        MessageBoxW(NULL, L"So", L"Who goes there?", MB_ICONQUESTION | MB_OK);
        break;
    }
    return TRUE;
}
