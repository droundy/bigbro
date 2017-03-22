#define UNICODE

#include <windows.h>
#include <wchar.h>
#include <stdio.h>
#include <stdlib.h>

#include "create_dlls.h"

static const wchar_t *helper_wstr = TEXT("helper.exe");
static const wchar_t *dll32_wstr = TEXT("bigbro32.dll");
static const wchar_t *dll64_wstr = TEXT("bigbro64.dll");

wchar_t helper_filename[4096];
wchar_t dll32_filename[4096];
wchar_t dll64_filename[4096];

static int pathcat(wchar_t *out, const wchar_t *path, const wchar_t *name) {
  for (int i=0; path[i]; i++) {
    *out++ = path[i];
    if (i > 2000) return 1;
  }
  for (int i=0; name[i]; i++) {
    *out++ = name[i];
  }
  *out = 0;
  return 0;
}

static int have_dll_paths = 0;

int init_dll_paths() {
  if (have_dll_paths) return 0;
  wchar_t szTempFileName[MAX_PATH];
  wchar_t path[MAX_PATH];

  //  Gets the temp path env string (no guarantee it's a valid path).
  DWORD pathlen = GetTempPathW(MAX_PATH,          // length of the buffer
                                path); // buffer for path
  if (pathlen > MAX_PATH || (pathlen == 0)) return 1;
  path[pathlen] = 0;

  //  Generates a temporary file name.
  UINT uRetVal = GetTempFileNameW(path, // directory for tmp files
                                  TEXT("DEMO"),     // temp file name prefix
                                  0,                // create unique name
                                  szTempFileName);  // buffer for name
  if (uRetVal == 0) return 1;

  if (pathcat(helper_filename, path, helper_wstr)) return 1;
  if (pathcat(dll32_filename, path, dll32_wstr)) return 1;
  if (pathcat(dll64_filename, path, dll64_wstr)) return 1;
  return 0;
}
