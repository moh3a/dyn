/*
initiation to the win32 api
*/

#include <windows.h>

int main(void)
{
    // win32 message box W for unicode encoding L""
    // MB_YESNOCANCEL for message box options yes/no/cancel | MB_ICONEXCLAMATION exclamation mark icon for message box
    MessageBoxW(
        NULL,
        L"hello moh3a!",
        L"hello",
        MB_YESNOCANCEL | MB_ICONEXCLAMATION);

    return EXIT_SUCCESS;
}
