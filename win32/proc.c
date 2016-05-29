/* Copyright (c) 2015, Jorge Acereda Maciá <jacereda@gmail.com>

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
#include <stdio.h>
#include <stdlib.h>

#include "proc.h"

enum procerr procRun(char *cmdline, int *rc) {
	STARTUPINFO si;
	PROCESS_INFORMATION pi;
	memset(&si, 0, sizeof(si));
	si.cb = sizeof(si);
  if (!CreateProcess(0, cmdline, 0, 0, 0, CREATE_SUSPENDED, 0, 0, &si, &pi)) {
    return ERR_PROC_FORK;
  }
  /* injectProcess(pi.hProcess); */
  if (ResumeThread(pi.hThread) != -1) {
    return ERR_PROC_FORK;
  }
  if (!WaitForSingleObject(pi.hThread, INFINITE) != WAIT_OBJECT_0) {
    return ERR_PROC_WAIT;
  }
	DWORD dword_return_code;
  if (!GetExitCodeProcess(pi.hProcess, &dword_return_code)) {
    return ERR_PROC_WAIT;
  }
  if (!CloseHandle(pi.hThread) || !CloseHandle(pi.hProcess)) {
    return ERR_PROC_FORK;
  }
	*rc = dword_return_code;
	return ERR_PROC_OK;
}
