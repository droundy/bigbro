#define UNICODE
#define _UNICODE

#include <windows.h>
#include <stdio.h>
#include <tchar.h>
#include <wchar.h>
#include <string.h>
#include <psapi.h>
#include <strsafe.h>

#include "path.h"

#define BUFSIZE 512

// The following function is taken from Microsoft's example source.
// Sadly, it is a rather contorted process to find the filename prior
// to Vista.  Kind of amazing!

// To support symlinks properly, I should probably write my own
// version of realpath, which tracks which symlinks were read in the
// process, like I did for posix systems.

// https://msdn.microsoft.com/en-us/library/aa366789.aspx
char *GetFileNameFromHandle(HANDLE hFile) {
  WCHAR pszFilename[MAX_PATH+1];
  HANDLE hFileMap;

  // Get the file size.
  DWORD dwFileSizeHi = 0;
  DWORD dwFileSizeLo = GetFileSize(hFile, &dwFileSizeHi);

  if( dwFileSizeLo == 0 && dwFileSizeHi == 0 ) {
     printf("Cannot map a file with a length of zero.\n");
     return 0;
  }

  // Create a file mapping object.
  hFileMap = CreateFileMappingW(hFile, NULL, PAGE_READONLY, 0, 1, NULL);
  if (!hFileMap) {
    printf("cannot CreateFileMappingW %p\n", hFile);
    return NULL;
  }

  // Create a file mapping to get the file name.
  void* pMem = MapViewOfFile(hFileMap, FILE_MAP_READ, 0, 0, 1);

  if (!pMem) {
    CloseHandle(hFileMap);
    printf("cannot MapViewOfFile\n");
    return NULL;
  }

  if (GetMappedFileNameW(GetCurrentProcess(), pMem,
                         pszFilename, MAX_PATH)) {
    // Translate path with device name to drive letters.
    WCHAR szTemp[BUFSIZE];
    szTemp[0] = '\0';

    if (GetLogicalDriveStrings(BUFSIZE-1, szTemp)) {
      WCHAR szName[MAX_PATH];
      WCHAR szDrive[3] = TEXT(" :");
      BOOL bFound = FALSE;
      WCHAR* p = szTemp;

      do {
        // Copy the drive letter to the template string
        *szDrive = *p;

        // Look up each device name
        if (QueryDosDevice(szDrive, szName, MAX_PATH)) {
          size_t uNameLen = _tcslen(szName);
          if (uNameLen < MAX_PATH) {
            bFound = _tcsnicmp(pszFilename, szName, uNameLen) == 0
              && *(pszFilename + uNameLen) == _T('\\');

            if (bFound) {
              // Reconstruct pszFilename using szTempFile
              // Replace device path with DOS path
              WCHAR szTempFile[MAX_PATH];
              StringCchPrintf(szTempFile,
                              MAX_PATH,
                              TEXT("%s%s"),
                              szDrive,
                              pszFilename+uNameLen);
              StringCchCopyN(pszFilename, MAX_PATH+1, szTempFile, _tcslen(szTempFile));
            }
          }
        }

        // Go to the next NULL character.
        while (*p++);
      } while (!bFound && *p); // end of string
    }
  }
  UnmapViewOfFile(pMem);
  CloseHandle(hFileMap);

  _tprintf(TEXT("File name is %s\n"), pszFilename);
  char *output = malloc(2*MAX_PATH+1);
  WideCharToMultiByte(CP_UTF8, 0, pszFilename, -1, output, 2*MAX_PATH+1, NULL, NULL);
  return output;
}
