#!/usr/bin/python3
from __future__ import division, print_function

# So far this is a place-holder script, but hopefully will build on
# windows when there is windows support.

import subprocess, os

for compiler in ['cl', 'x86_64-w64-mingw32-gcc', 'cc']:
    try:
        subprocess.call([compiler, '--version'])
        cc = compiler
        print('using',cc,'compiler')
        break
    except:
        print('NOT using',compiler,'compiler')

for linker in ['x86_64-w64-mingw32-gcc', 'link', 'ld']:
    try:
        subprocess.call([linker, '--version'])
        print('using',linker,'linker')
        break
    except:
        print('NOT using',linker,'linker')

print("This is a test under windows")

if compiler == 'cl':
    cflags = []
    objout = lambda fname: '-Fo'+fname
    exeout = lambda fname: '-Fe'+fname
else:
    cflags = ['-std=c99', '-g']
    objout = lambda fname: '-o='+fname
    exeout = objout

def compile(cfile):
    cmd = [cc, '-c', '-O2'] + cflags + [objout(cfile[:-2]+'.obj'), cfile]
    print(' '.join(cmd))
    return subprocess.call(cmd)

cfiles = ['bigbro-windows.c', 'fileaccesses.c']

for c in cfiles:
    assert(not compile(c))

# use cc for doing the linking, since I understand its options
cmd = [cc, exeout('bigbro.exe')] + [c[:-1]+'obj' for c in cfiles]
print(' '.join(cmd))
assert(not subprocess.call(cmd))
