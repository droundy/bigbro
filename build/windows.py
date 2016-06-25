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

from __future__ import division, print_function

import sys, subprocess, os, binary2header

cc = 'x86_64-w64-mingw32-gcc'
print('trying', cc)
subprocess.call([cc, '--version'])
cc32 = 'i686-w64-mingw32-gcc'
print('trying', cc32)
subprocess.call([cc32, '--version'])

cflags = ['-std=c99']
objout = lambda fname: '-o'+fname
exeout = objout

def compile(cfile):
    cmd = [cc, '-c', '-O2'] + cflags + [objout(cfile[:-2]+'.obj'), cfile]
    print(' '.join(cmd))
    return subprocess.call(cmd)
def compile32(cfile):
    cmd = [cc32, '-c', '-O2'] + cflags + [objout(cfile[:-2]+'32.obj'), cfile]
    print(' '.join(cmd))
    return subprocess.call(cmd)


libraryfiles = ['bigbro-windows.c', 'win32/inject.c',
                'win32/queue.c', 'win32/create_dlls.c', 'win32/dll_paths.c']
cfiles = libraryfiles + ['fileaccesses.c']
dll_cfiles = ['win32/inject.c', 'win32/dll.c', 'win32/patch.c', 'win32/hooks.c',
              'win32/queue.c', 'win32/dll_paths.c']

if 'x86' in sys.argv or 'amd64' not in sys.argv:
    # first build the helper executable
    cmd = [cc32, '-c', '-Os', objout('win32/helper.obj'), 'win32/helper.c']
    print(' '.join(cmd))
    assert(not subprocess.call(cmd))
    cmd = [cc32, exeout('win32/helper.exe'), 'win32/helper.obj']
    print(' '.join(cmd))
    assert(not subprocess.call(cmd))

    # now convert this executable into a header file
    binary2header.convertFile('win32/helper.exe', 'win32/helper.h', 'helper')
    print('I have now created win32/helper.h')

    for c in dll_cfiles:
        assert(not compile32(c))
    cmd = [cc32, '-shared', '-o', 'bigbro32.dll'] + [c[:-2]+'32.obj' for c in dll_cfiles] + ['-lntdll', '-lpsapi']
    print(' '.join(cmd))
    assert(not subprocess.call(cmd))

if 'amd64' in sys.argv or 'x86' not in sys.argv:

    for c in cfiles + dll_cfiles:
        assert(not compile(c))

    cmd = [cc, '-shared', '-o', 'bigbro64.dll'] + [c[:-2]+'.obj' for c in dll_cfiles] + ['-lntdll', '-lpsapi']
    print(' '.join(cmd))
    assert(not subprocess.call(cmd))

    # use cc for doing the linking
    cmd = [cc, exeout('bigbro.exe')] + [c[:-1]+'obj' for c in cfiles]
    print(' '.join(cmd))
    assert(not subprocess.call(cmd))
