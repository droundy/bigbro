#!/bin/sh

set -ev

(x86_64-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o bigbro-windows.obj bigbro-windows.c)

(x86_64-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o fileaccesses.obj fileaccesses.c)

(i686-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/dll32.obj win32/dll.c)

(i686-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/dll_paths32.obj win32/dll_paths.c)

(i686-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/hooks32.obj win32/hooks.c)

(i686-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/helper32.obj win32/helper.c)

(i686-w64-mingw32-gcc -o win32/helper.exe win32/helper32.obj)

(python3 build/binary2header.py win32/helper.exe win32/helper.h helper)

(i686-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/inject32.obj win32/inject.c)

(i686-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/patch32.obj win32/patch.c)

(i686-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/queue32.obj win32/queue.c)

(i686-w64-mingw32-gcc -shared -o bigbro32.dll win32/inject32.obj win32/dll32.obj win32/patch32.obj win32/hooks32.obj win32/queue32.obj win32/dll_paths32.obj -lntdll -lpsapi)

(python3 build/binary2header.py bigbro32.dll win32/bigbro32.h bigbro32dll)

(x86_64-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/dll.obj win32/dll.c)

(x86_64-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/dll_paths.obj win32/dll_paths.c)

(x86_64-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/hooks.obj win32/hooks.c)

(x86_64-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/inject.obj win32/inject.c)

(x86_64-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/patch.obj win32/patch.c)

(x86_64-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/queue.obj win32/queue.c)

(x86_64-w64-mingw32-gcc -shared -o bigbro64.dll win32/inject.obj win32/dll.obj win32/patch.obj win32/hooks.obj win32/queue.obj win32/dll_paths.obj -lntdll -lpsapi)

(python3 build/binary2header.py bigbro64.dll win32/bigbro64.h bigbro64dll)

(x86_64-w64-mingw32-gcc  -Wall -Werror -std=c99 -g -O2 -c -o win32/create_dlls.obj win32/create_dlls.c)

(rm -f libbigbro-windows.a && x86_64-w64-mingw32-ar rc libbigbro-windows.a bigbro-windows.obj win32/inject.obj win32/queue.obj win32/create_dlls.obj win32/dll_paths.obj && x86_64-w64-mingw32-ranlib libbigbro-windows.a)

(x86_64-w64-mingw32-gcc -o bigbro.exe fileaccesses.obj libbigbro-windows.a)

