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

#include <stdint.h>
#include <windows.h>
#include <limits.h>
#include <assert.h>

#include <shellapi.h>
#include "../errors.h"
#include "helper.h"

#include "inject.h"

WINBASEAPI DWORD WINAPI GetProcessIdOfThread(HANDLE Thread);

#ifndef PATH_MAX
#define PATH_MAX 4096
#endif

void injectProcess(HANDLE proc) {
	HANDLE tid;
	BOOL is32;
	FARPROC addr;
	LPVOID arg;
	char dll[PATH_MAX];
	char *ext = 0;
	DWORD rc;
	extern IMAGE_DOS_HEADER __ImageBase;
	assert(proc);
	memset(dll, 0, sizeof(dll));
	assert(GetModuleFileNameA((HMODULE)&__ImageBase, dll, sizeof(dll)));
	if (!ext)
		ext = strstr(dll, ".exe");
	if (!ext)
		ext = strstr(dll, ".dll");
	if (!ext)
		ext = dll + strlen(dll);
	assert(IsWow64Process(proc, &is32));
	assert(0 != (arg = VirtualAllocEx(proc, 0, strlen(dll) + 1,
				       MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE)));
	if (strcmp(ext, ".dll"))
		memcpy(ext, is32 ? "32.dll" : "64.dll", 6);
        debugprintf("dll name is %s\n", dll);
	if (is32) {
		STARTUPINFO si;
		PROCESS_INFORMATION pi;
		const char * helpername = "fsatracehelper.exe";
		char helper[PATH_MAX];
		char * p;
		memset(&si, 0, sizeof(si));
		memset(&pi, 0, sizeof(pi));
		si.cb = sizeof(si);
		memcpy(helper, dll, strlen(dll)+1);
		p = strrchr(helper, '\\');
		memcpy(p+1, helpername, strlen(helpername)+1);
                debugprintf("helper is %s\n", helper);
		assert(CreateProcessA(0, helper, 0, 0, 0, 0, 0, 0, &si, &pi));
		assert(WAIT_OBJECT_0 == WaitForSingleObject(pi.hProcess, INFINITE));
		assert(GetExitCodeProcess(pi.hProcess, &rc));
		addr = (FARPROC)(uintptr_t)rc;
	}
	else
		addr = GetProcAddress(GetModuleHandle("kernel32.dll"), "LoadLibraryA");
	assert(addr);
	assert(WriteProcessMemory(proc, arg, dll, strlen(dll) + 1, NULL));
        debugprintf("am creating remote thread...\n");
        tid = CreateRemoteThread(proc, 0, 0, (LPTHREAD_START_ROUTINE)addr, arg, 0, 0);
	assert(0 != tid);
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
