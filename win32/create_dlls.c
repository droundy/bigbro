#define UNICODE

#include <windows.h>
#include <wchar.h>
#include <stdio.h>
#include <stdlib.h>

#include "helper.h"
#include "bigbro32.h"
#include "bigbro64.h"
#include "create_dlls.h"

static int write_file(const wchar_t *fname, const unsigned char *content, size_t size) {
  SECURITY_DESCRIPTOR sd;
  InitializeSecurityDescriptor(&sd, SECURITY_DESCRIPTOR_REVISION);
  HANDLE h = CreateFile(fname, GENERIC_WRITE | GENERIC_EXECUTE,
                        0,                    // do not share
                        NULL,                 // default security
                        CREATE_ALWAYS,        // overwrite existing
                        FILE_ATTRIBUTE_NORMAL,// normal file
                        NULL);                // no template
  if (h == INVALID_HANDLE_VALUE) {
    return 1;
  }

  DWORD dwBytesWritten = 0;
  BOOL success = WriteFile(h, content, size, &dwBytesWritten, NULL);
  CloseHandle(h);
  if (!success) {
    // wprintf(TEXT("Trouble writing to file %s\n"), fname);
  }
  return !success;
}

static int have_dlls = 0;

int create_dlls() {
  if (have_dlls) return 0;
  if (init_dll_paths()) return 1;

  if (write_file(helper_filename, helper, sizeof(helper))) return 1;
  if (write_file(dll32_filename, bigbro32dll, sizeof(bigbro32dll))) return 1;
  if (write_file(dll64_filename, bigbro64dll, sizeof(bigbro64dll))) return 1;

  /* HMODULE hh = LoadLibraryW(dll64_filename); */
  /* if (hh == 0) { */
  /*   printf("UNABLE TO LOAD LIBRARY WE JUST WROTE!\n"); */
  /*   DWORD err = GetLastError(); */
  /*   switch (err) { */
  /*   case 126: */
  /*     wprintf(L"The specified module '%s' could not be found!\n", dll64_filename); */
  /*     break; */
  /*   case 1114: */
  /*     wprintf(L"The '%s' initialization routine failed!\n", dll64_filename); */
  /*     break; */
  /*   default: */
  /*     printf("error is %ld\n", err); */
  /*   } */
  /* } */

  have_dlls = 1;
  return 0;
}
