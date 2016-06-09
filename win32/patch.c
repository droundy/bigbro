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
#include <string.h>
#include <windows.h>
#include <psapi.h>
#include <assert.h>
#include "../errors.h"
#include "patch.h"

static DWORD s_hooked;

static IMAGE_IMPORT_DESCRIPTOR *imports(IMAGE_DOS_HEADER *dh) {
	char *base = (char *)dh;
	IMAGE_NT_HEADERS *nth = (IMAGE_NT_HEADERS *)(base + dh->e_lfanew);
	IMAGE_DATA_DIRECTORY *impdd =
		nth->OptionalHeader.DataDirectory + IMAGE_DIRECTORY_ENTRY_IMPORT;
	assert(dh->e_lfanew);
	assert(nth);
	assert(impdd);
	return impdd && impdd->VirtualAddress
		? (IMAGE_IMPORT_DESCRIPTOR *)(base + impdd->VirtualAddress)
		: 0;
}

static IMAGE_THUNK_DATA *lookup2(char *base,
                                 IMAGE_THUNK_DATA *td,
                                 IMAGE_THUNK_DATA *otd,
                                 const char *nm) {
  while (otd->u1.AddressOfData) {
    IMAGE_IMPORT_BY_NAME *name =
      (IMAGE_IMPORT_BY_NAME *)(otd->u1.AddressOfData + base);
    if (otd->u1.Ordinal & IMAGE_ORDINAL_FLAG)
      debugprintf("   ordinal1\n");
    else {
      debugprintf("  name: %s %p\n", name->Name, td->u1.Function);
      if (0 == strcmp((char*)name->Name, nm))
        return td;
    }
    otd++;
    td++;
  }
  return 0;
}

static IMAGE_THUNK_DATA *lookup(IMAGE_DOS_HEADER *dh, const char *nm) {
  char *base = (char *)dh;
  IMAGE_IMPORT_DESCRIPTOR *id = imports(dh);
  if (!id) return 0;
  while (id->Name) {
    if (id->FirstThunk && id->OriginalFirstThunk) {
      IMAGE_THUNK_DATA *d =
        lookup2(base,
                (IMAGE_THUNK_DATA*)(id->FirstThunk + base),
                (IMAGE_THUNK_DATA*)(id->OriginalFirstThunk + base),
                nm);
      debugprintf(" import %s\n", id->Name + base);
      if (d)
        return d;
    }
    id++;
  }
  return 0;
}

static void modpatch(IMAGE_DOS_HEADER *dh,
                     void *orig,
                     void *hook,
                     void **preal,
                     const char *nm) {
	IMAGE_THUNK_DATA *td = lookup(dh, nm);
	if (td && orig == (void *)td->u1.Function) {
		DWORD o;
		debugprintf("   patching %s %p %p -> %p\n", nm, td->u1.Function, orig, hook);
		*preal = (void *)td->u1.Function;
		assert(VirtualProtect(td, sizeof(*td), PAGE_EXECUTE_READWRITE, &o));
		td->u1.Function = (uintptr_t)hook;
		assert(VirtualProtect(td, sizeof(*td), o, &o));
	}
}

void patchInstall(void *orig, void *hook, void **preal, const char *nm) {
	HMODULE mod[4096];
	DWORD n;
	DWORD i;
	extern IMAGE_DOS_HEADER __ImageBase;
	assert(EnumProcessModules(GetCurrentProcess(), mod, sizeof(mod), &n));
	n /= sizeof(HMODULE);
	debugprintf("orig %s %p\n", nm, orig);
	for (i = 0; i < n; i++) {
		HMODULE m = mod[i];
		char mname[4096];
		assert(GetModuleFileNameA(m, mname, sizeof(mname)));
		debugprintf("module %s\n", mname);
		if (m != (HMODULE)&__ImageBase)
			modpatch((IMAGE_DOS_HEADER *)m, orig, hook, preal, nm);
	}
	debugprintf("modules patched\n");
}

int patchInstalled() {
	int ret;
	assert(s_hooked);
	ret = (int)(intptr_t)TlsGetValue(s_hooked);
	assert(TlsSetValue(s_hooked, (void *)1));
	return ret;
}

void patchInit() {
	assert(0 != (s_hooked = TlsAlloc()));
}

void patchTerm() {
	assert(TlsFree(s_hooked));
}
