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

#define UNICODE

#include <stdint.h>
#include <windows.h>
#include <limits.h>
#include <assert.h>

#include <shellapi.h>
#include "../errors.h"

#include "inject.h"
#include "create_dlls.h"

WINBASEAPI DWORD WINAPI GetProcessIdOfThread(HANDLE Thread);

#ifndef PATH_MAX
#define PATH_MAX 4096
#endif

void injectProcess(HANDLE proc) {
  init_dll_paths();
  HANDLE tid;
  BOOL is32;
  FARPROC addr;
  LPVOID arg;
  wchar_t *dll;
  DWORD rc;
  assert(proc);
  assert(IsWow64Process(proc, &is32));
  if (is32) {
    dll = dll32_filename;
    STARTUPINFO si;
    PROCESS_INFORMATION pi;
    assert(CreateProcessW(0, helper_filename, 0, 0, 0, 0, 0, 0, &si, &pi));
    assert(WAIT_OBJECT_0 == WaitForSingleObject(pi.hProcess, INFINITE));
    assert(GetExitCodeProcess(pi.hProcess, &rc));
    addr = (FARPROC)(uintptr_t)rc;
  } else {
    dll = dll64_filename;
    addr = GetProcAddress(GetModuleHandleW(TEXT("kernel32.dll")), "LoadLibraryW");
  }
  assert(addr);
  arg = VirtualAllocEx(proc, 0, 2*(wcslen(dll) + 1),
                       MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
  assert(arg);
  wchar_t stupidbuffer[5000];
  for (int i=0; i<4000; i++) stupidbuffer[i] = 0;
  memcpy(stupidbuffer, dll, 2*(wcslen(dll) + 1));
  assert(WriteProcessMemory(proc, arg, dll, 2*(wcslen(dll) + 1), NULL));
  debugprintf("am creating remote thread...\n");
  tid = CreateRemoteThread(proc, 0, 0, (LPTHREAD_START_ROUTINE)addr, arg, 0, 0);
  assert(tid);
  assert(-1 != ResumeThread(tid));
  assert(WAIT_OBJECT_0 == WaitForSingleObject(tid, INFINITE));
  assert(CloseHandle(tid));
  debugprintf("have finished waiting for the remote thread...\n");
}

void injectThread(HANDLE th) {
  HANDLE h;
  assert(0 != (h = OpenProcess(PROCESS_ALL_ACCESS, TRUE,
                               GetProcessIdOfThread(th))));
  injectProcess(h);
  assert(CloseHandle(h));
}
