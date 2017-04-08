#!/usr/bin/python3

# bigbro filetracking library
# Copyright (C) 2015,2016 David Roundy
#
# This program is free software; you can redistribute it and/or
# modify it under the terms of the GNU General Public License as
# published by the Free Software Foundation; either version 2 of the
# License, or (at your option) any later version.
#
# This program is distributed in the hope that it will be useful, but
# WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
# General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; if not, write to the Free Software
# Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
# 02110-1301 USA

import string, os, glob, sys, importlib

def is_in_path(program):
    """ Does the program exist in the PATH? """
    def is_exe(fpath):
        return os.path.isfile(fpath) and os.access(fpath, os.X_OK)
    fpath, fname = os.path.split(program)
    if fpath:
        return is_exe(program)
    else:
        for path in os.environ["PATH"].split(os.pathsep):
            path = path.strip('"')
            exe_file = os.path.join(path, program)
            if is_exe(exe_file):
                return True
    return False

platform = sys.platform
if platform == 'linux2':
    platform = 'linux'

os.system('rm -rf testing-flags');
os.mkdir('testing-flags');
with open('testing-flags/test.c', 'w') as f:
    f.write("""int main() {
  return 0;
}
""")

cc = 'gcc'

cflags = '$CFLAGS'
for flag in ['-O2', '-Wall', '-Werror', '-std=c99', '-g', '-mtune=native', '-fpic']:
    if not os.system('cd testing-flags && %s %s %s -c test.c' %
                     (cc, cflags, flag)):
        cflags += ' ' + flag
    else:
        print('# %s cannot use flag: %s' % (cc, flag))

print('# cc=', repr(cc)) # in future, we should set this dynamically
print('# cflags=', repr(cflags))

print("""
| %s %s -c bigbro-%s.c
< syscalls/linux.h
> bigbro-%s.o

# We need to remove libbigbro.a before running ar, because otherwise
# it will be added to, rather than replaced.

| rm -f libbigbro.a && ${AR-ar} rc libbigbro.a bigbro-%s.o && ${RANLIB-ranlib} libbigbro.a
< bigbro-%s.o
> libbigbro.a
""" % (cc, cflags, platform, platform, platform, platform))

print("""
| %s %s -static -o bigbro -L. fileaccesses.c -lbigbro
< libbigbro.a
> bigbro
""" % (cc, cflags))

print("""
| %s %s -shared -o libbigbro.so bigbro-%s.o
< bigbro-%s.o
> libbigbro.so
""" % (cc, cflags, platform, platform))


winlibraryfiles = ['bigbro-windows.c', 'win32/inject.c',
                   'win32/queue.c', 'win32/create_dlls.c', 'win32/dll_paths.c']
wincfiles = winlibraryfiles + ['fileaccesses.c']
dll_cfiles = ['win32/inject.c', 'win32/dll.c', 'win32/patch.c', 'win32/hooks.c',
              'win32/queue.c', 'win32/dll_paths.c']

if is_in_path('i686-w64-mingw32-gcc'):
    print('\n# We have a 32-bit mingw compiler, so let us cross-compile for windows!\n')

    cflags = ''
    for flag in ['-Wall', '-Werror', '-std=c99', '-g', '-O2']:
        if not os.system('cd testing-flags && i686-w64-mingw32-gcc %s %s -c test.c' %
                         (cflags, flag)):
            cflags += ' ' + flag
    else:
        print('# i686-w64-mingw32-gcc cannot use flag: %s' % flag)

    print('''
# first build the helper executable
| i686-w64-mingw32-gcc %s -c -o win32/helper32.obj win32/helper.c

| i686-w64-mingw32-gcc -o win32/helper.exe win32/helper32.obj
< win32/helper32.obj

# now convert this executable into a header file
| python3 build/binary2header.py win32/helper.exe win32/helper.h helper
< win32/helper.exe
''' % (cflags))

    for c in dll_cfiles:
          print("""
| i686-w64-mingw32-gcc %s -c -o %s32.obj %s
""" % (cflags, c[:-2], c))

    print('''
| i686-w64-mingw32-gcc -shared -o bigbro32.dll %s -lntdll -lpsapi'''
          % ' '.join([c[:-2]+'32.obj' for c in dll_cfiles]))
    for c in dll_cfiles:
        print("< %s32.obj" % c[:-2])

if is_in_path('x86_64-w64-mingw32-gcc'):
    print('\n# We have a 64-bit mingw compiler, so let us cross-compile for windows!\n')

    cflags = ''
    for flag in ['-Wall', '-Werror', '-std=c99', '-g', '-O2']:
        if not os.system('cd testing-flags && x86_64-w64-mingw32-gcc %s %s -c test.c' %
                         (cflags, flag)):
            cflags += ' ' + flag
    else:
        print('# x86_64-w64-mingw32-gcc cannot use flag: %s' % flag)

    for c in set(dll_cfiles+wincfiles):
          print("""
| x86_64-w64-mingw32-gcc %s -c -o %s.obj %s
> %s.obj
""" % (cflags, c[:-2], c, c[:-2]))
          if c == 'win32/create_dlls.c':
              print('< win32/bigbro32.h')
              print('< win32/bigbro64.h')
              print('< win32/helper.h')

    print('''
| x86_64-w64-mingw32-gcc -shared -o bigbro64.dll %s -lntdll -lpsapi'''
          % ' '.join([c[:-2]+'.obj' for c in dll_cfiles]))
    for c in dll_cfiles:
        print("< %s.obj" % c[:-2])

    print('''
# convert the dlls into into header files
| python3 build/binary2header.py bigbro32.dll win32/bigbro32.h bigbro32dll
< bigbro32.dll
| python3 build/binary2header.py bigbro64.dll win32/bigbro64.h bigbro64dll
< bigbro64.dll
''')

    print("""
| x86_64-w64-mingw32-gcc -o bigbro.exe fileaccesses.obj libbigbro-windows.a
< fileaccesses.obj
< libbigbro-windows.a
""")
    for c in wincfiles:
        print("< %s.obj" % c[:-2])
    print("""
# We need to remove libbigbro.a before running ar, because otherwise
# it will be added to, rather than replaced.

| rm -f libbigbro-windows.a && x86_64-w64-mingw32-gcc-ar rc libbigbro-windows.a %s && x86_64-w64-mingw32-gcc-ranlib libbigbro-windows.a
> libbigbro-windows.a
""" % (' '.join([c[:-1]+'obj' for c in winlibraryfiles])))
    for c in winlibraryfiles:
        print("< %s.obj" % c[:-2])


    for c in glob.glob('tests/*.c'):
        base = c[:-2]
        m = importlib.import_module('tests.'+base[6:])
        if 'skip_windows' in dir(m):
            print('# skipping test', base, 'not supported by windows')
        else:
            print("""
| x86_64-w64-mingw32-gcc %s -o %s-test.exe %s"""
                  % (cflags, c[:-2], c))

os.system('rm -rf testing-flags')

if is_in_path('sass'):
    print('''
| sass -I. web/style.scss web/style.css
> web/style.css
C .sass-cache
''')
else:
    print("# no sass, so we won't build style.css")

if is_in_path('doxygen'):
    print('''
| doxygen build/Doxyfile
> web/doxy/bigbro_8h.html
< build/doxy/header.html

| cp web/doxy/bigbro_8h.html web/documentation.html
< web/doxy/bigbro_8h.html
''')

if is_in_path('cargo'):
        print('''
| cargo build && cargo doc
< syscalls/linux.h
c ~
c .tum
c .pyc
C bench
C tests
C web

| cargo build --release
< target/debug/test-bigbro
c ~
c .tum
c .pyc
C bench
C tests
C web

| cp -a target/doc web/
> web/doc/bigbro/index.html
< target/doc/bigbro/index.html
''')
else:
    print('# no cargo, so cannot build using rust')

