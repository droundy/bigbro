/* Copyright (c) 2015, Jorge Acereda Maciá <jacereda@gmail.com>
   Copyright (C) 2016 David Roundy <daveroundy@gmail.com>

   Permission to use, copy, modify, and/or distribute this software
   for any purpose with or without fee is hereby granted, provided
   that the above copyright notice and this permission notice appear
   in all copies.

   THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL
   WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED
   WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
   AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR
   CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
   OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
   NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
   CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE. */

#include <windows.h>
#include <assert.h>
#include "patch.h"
#include "../errors.h"
#include "hooks.h"
#include "queue.h"

HANDLE pipe_Wr = 0;

static void * resolve(const char * name) {
  void * ret;
  static HANDLE dll = 0;
  if (!dll) {
    dll = GetModuleHandleA("ntdll.dll");
    assert(dll);
  }
  ret = GetProcAddress(dll, name);
  assert(ret);
  return ret;
}

INT APIENTRY DllMain(HMODULE hDLL, DWORD Reason, LPVOID Reserved) {
  switch (Reason) {
  case DLL_PROCESS_ATTACH:
    debugprintf("I am attaching my dll\n");
    char *shm_name = malloc(50);
    DWORD ret = GetEnvironmentVariable("bigbro_shm", shm_name, 50);
    if (ret == 0) {
      debugprintf("There is no bigbro_sm environment variable!\n");
      return FALSE;
    } else if (ret >= 50) {
      debugprintf("There is a long bigbro_pipe environment variable that seems like an attack!\n");
      return FALSE;
    } else {
      if (globalQueueInit(shm_name)) {
        debugprintf("error with shm opening '%s'!\n", shm_name);
        return FALSE;
      }
    }
    patchInit();
    hooksInit(resolve);
    break;
  case DLL_PROCESS_DETACH:
    debugprintf("I am detaching my dll\n");
    patchTerm();
    /* emitTerm(); */
    break;
  }
  return TRUE;
}
