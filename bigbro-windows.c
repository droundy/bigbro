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

#include "bigbro.h"
#include "hashset.h"

int bigbro(const char *workingdir, pid_t *child_ptr,
           int stdoutfd, int stderrfd, char *envp[],
           char *cmdline, char ***read_from_directories,
           char ***read_from_files, char ***written_to_files) {
  STARTUPINFO si;
  PROCESS_INFORMATION pi;
  memset(&si, 0, sizeof(si));
  si.cb = sizeof(si);
  if (!CreateProcess(0, cmdline, 0, 0, 0, CREATE_SUSPENDED, 0, 0, &si, &pi)) {
    return -1;
  }
  /* injectProcess(pi.hProcess); */
  if (ResumeThread(pi.hThread) != -1) {
    return -1;
  }
  if (!WaitForSingleObject(pi.hThread, INFINITE) != WAIT_OBJECT_0) {
    return -1;
  }
  DWORD dword_return_code;
  if (!GetExitCodeProcess(pi.hProcess, &dword_return_code)) {
    return -1;
  }
  if (!CloseHandle(pi.hThread) || !CloseHandle(pi.hProcess)) {
    return 1;
  }
  hashset read, readdir, written;
  init_hashset(&read, 1024);
  init_hashset(&readdir, 1024);
  init_hashset(&written, 1024);
  *read_from_files = hashset_to_array(&read);
  *read_from_directories = hashset_to_array(&readdir);
  *written_to_files = hashset_to_array(&written);
  return dword_return_code;
}
