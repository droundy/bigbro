/* bigbro filetracking library
   Copyright (C) 2016 David Roundy

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
   02110-1301 USA */

#include <windows.h>
#include <stdio.h>
#include <stdlib.h>

#include "bigbro.h"
#include "hashset.h"

#include "win32/inject.h"

// copy the string and return a pointer to the byte in dest *after*
// the null character we write.
static inline char *copy_string(char *dest, const char *input) {
  for (int i=0; input[i]; i++) {
    *dest++ = input[i];
  }
  *dest++ = 0;
  return dest;
}

int bigbro(const char *workingdir, pid_t *child_ptr,
           int stdoutfd, int stderrfd, char *envp[],
           char *cmdline, char ***read_from_directories,
           char ***read_from_files, char ***written_to_files) {
  HANDLE pipe_Rd, pipe_Wr;
  {
    SECURITY_ATTRIBUTES saAttr;
    // Set the bInheritHandle flag so pipe handles are inherited.
    saAttr.nLength = sizeof(SECURITY_ATTRIBUTES);
    saAttr.bInheritHandle = TRUE;
    saAttr.lpSecurityDescriptor = NULL;
    if (!CreatePipe(&pipe_Rd, &pipe_Wr, &saAttr, 0) ) {
      return -1;
    }
  }
  char *pipe_Wr_string = malloc(50);
  char *new_windows_env = 0;
  snprintf(pipe_Wr_string, 50, "%p", pipe_Wr);
  if (envp) {
    // create an environment with the desired environment variables,
    // plus one more to carry the value of pipe_Wr.  This is trickier
    // than on posix, since windows wants a null-separated array of
    // char, rather than a null-terminated array of char *.  For
    // uniformity, bigbro wants the latter.
    int size = 1; // 1 for the final null
    for (char **e = envp; *e; e++) {
      size += strlen(*e) + 1;
    }
    size += strlen("bigbro_pipe=") + strlen(pipe_Wr_string) + 1;
    char *new_windows_env = (char *)calloc(size, 1);
    // here we join together all these strings into one big happy
    // family.
    char *location = new_windows_env;
    for (char **e = envp; *e; e++) location = copy_string(location, *e);
    location = copy_string(location, "bigbro_pipe=");
    location--; // overwrite the null
    location = copy_string(location, pipe_Wr_string);
  } else {
    // FIXME BUG HERE! the following introduces a race condition: if
    // we call bigbro twice simultaneously, it is possible that one of
    // the two processes will end up sending its info to the wrong
    // pipe.  However, right now I am just trying to get something
    // working, and this is easier.
    SetEnvironmentVariable("bigbro_pipe", pipe_Wr_string);
  }

  STARTUPINFO si;
  PROCESS_INFORMATION pi;
  memset(&si, 0, sizeof(si));
  si.cb = sizeof(si);
  // want to pass pipe_Wr value in the environment...
  if (!CreateProcess(0, cmdline, 0, 0, 0, CREATE_SUSPENDED, 0, 0, &si, &pi)) {
    return -1;
  }
  injectProcess(pi.hProcess, pipe_Wr); // FIXME
  CloseHandle(pipe_Wr);
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
