// START A NOTEPAD PROCESS

#include <stdio.h>
#include <windows.h>

int main(void)
{
    // Specifies the window station, desktop, standard handles, and appearance of the main window for a process at creation time.
    STARTUPINFOW si = {0}; // source https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfow

    // Contains information about a newly created process and its primary thread.
    PROCESS_INFORMATION pi = {0}; // source https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information

    /*
        BOOL CreateProcessW(
            [in, optional]      LPCWSTR               lpApplicationName,
            [in, out, optional] LPWSTR                lpCommandLine,
            [in, optional]      LPSECURITY_ATTRIBUTES lpProcessAttributes,
            [in, optional]      LPSECURITY_ATTRIBUTES lpThreadAttributes,
            [in]                BOOL                  bInheritHandles,
            [in]                DWORD                 dwCreationFlags,
            [in, optional]      LPVOID                lpEnvironment,
            [in, optional]      LPCWSTR               lpCurrentDirectory,
            [in]                LPSTARTUPINFOW        lpStartupInfo,
            [out]               LPPROCESS_INFORMATION lpProcessInformation
        );

        source: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw
    */
    if (!CreateProcessW(
            L"C:\\Windows\\System32\\notepad.exe",
            NULL,
            NULL,
            NULL,
            FALSE,
            BELOW_NORMAL_PRIORITY_CLASS,
            NULL,
            NULL,
            &si,
            &pi))
    {
        printf(
            "(-) Failed to create process, error: %ld",
            GetLastError() // Retrieves the calling thread's last-error code value. The last-error code is maintained on a per-thread basis. Multiple threads do not overwrite each other's last-error code.
        );
        return EXIT_FAILURE;
    }

    printf("(+) Process started! pid: %ld", pi.dwProcessId);
    return EXIT_SUCCESS;
}
