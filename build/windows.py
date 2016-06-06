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

import subprocess, os, binary2header

for compiler in ['cl', 'x86_64-w64-mingw32-gcc', 'cc']:
    try:
        subprocess.call([compiler, '--version'])
        cc = compiler
        print('using',cc,'compiler')
        break
    except:
        print('NOT using',compiler,'compiler')

for compiler in [r'C:\"Program Files (x86)\Microsoft Visual Studio 14.0"\VC\BIN\x86\cl.exe',
                 'i686-w64-mingw32-gcc', 'cc']:
    try:
        subprocess.call([compiler, '--version'])
        cc32 = compiler
        print('using',cc,'32-bit compiler')
        break
    except:
        print('NOT using',compiler,'32-bit compiler')

for linker in ['x86_64-w64-mingw32-gcc', 'link', 'ld']:
    try:
        subprocess.call([linker, '--version'])
        print('using',linker,'linker')
        break
    except:
        print('NOT using',linker,'linker')

if compiler == 'cl':
    cflags = []
    objout = lambda fname: '-Fo'+fname
    exeout = lambda fname: '-Fe'+fname
else:
    cflags = ['-std=c99', '-g']
    objout = lambda fname: '-o'+fname
    exeout = objout

def compile(cfile):
    cmd = [cc, '-c', '-O2'] + cflags + [objout(cfile[:-2]+'.obj'), cfile]
    print(' '.join(cmd))
    return subprocess.call(cmd)

cfiles = ['bigbro-windows.c', 'fileaccesses.c']
compile_only_files = ['win32/patch.c', 'win32/inject.c', 'win32/helper.c']

# first link the helper executable
cmd = [cc32, '-Os', exeout('win32/helper.exe'), 'win32/helper.c']
print(' '.join(cmd))
assert(not subprocess.call(cmd))

binary2header.convertFile('win32/helper.exe', 'win32/helper.h', 'helper')
print('I have now created win32/helper.h')

for c in cfiles + compile_only_files:
    assert(not compile(c))

# use cc for doing the linking, since I understand its options
cmd = [cc, exeout('bigbro.exe')] + [c[:-1]+'obj' for c in cfiles]
print(' '.join(cmd))
assert(not subprocess.call(cmd))
