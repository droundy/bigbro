#!/usr/bin/python3
from __future__ import division, print_function

# So far this is a place-holder script, but hopefully will build on
# windows when there is windows support.

import subprocess, os

if not subprocess.call(['x86_64-w64-mingw32-gcc', '--version']):
    cc = 'x86_64-w64-mingw32-gcc'
    print('using mingw-w64 compiler')
else:
    cc = 'cl'
    print('using cl, which might be visual studio compiler')

print("This is a test under windows")

def compile(cfile):
    cmd = [cc, '-c', '-O2', '-g', '-o', cfile[:-2]+'.obj', cfile]
    print(' '.join(cmd))
    return subprocess.call(cmd)

assert(not compile('win32/proc.c'))
