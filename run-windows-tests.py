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

from __future__ import print_function

import glob, os, subprocess, importlib, sys, time, shutil

if sys.version_info < (3,2):
    print('Please run this script with python 3.2 or newer.')
    exit(1)

if 'perf_counter' in dir(time):
    perf_counter = time.perf_counter
else:
    perf_counter = time.time

benchmark = 'bench' in sys.argv

platform = sys.platform
if platform == 'linux2':
    platform = 'linux'

for f in glob.glob('tests/*.test') + glob.glob('*.gcno') + glob.glob('*.gcda'):
    os.remove(f)

if platform == 'linux':
    print('NOT building bigbro by running build/cross-windows.sh...')
    print('===================================================')
    #assert not os.system('sh build/cross-windows.sh')
else:
    print('building bigbro by running build/windows.py...')
    print('============================================')

    assert not os.system('python build/windows.py')

print('I finished building bigbro.exe and will now run tests!')

numfailures = 0
numpasses = 0

have_symlinks = True

def create_clean_tree(preppy='this file does not exist'):
    for tmp in glob.glob('tmp*'):
        if os.path.isdir(tmp):
            shutil.rmtree(tmp)
        else:
            os.remove(tmp)
    os.mkdir('tmp')
    os.mkdir('tmp/subdir1')
    os.mkdir('tmp/subdir1/deepdir')
    os.mkdir('tmp/subdir2')
    with open('tmp/subdir2/test', 'w') as f:
        f.write('test\n')
    with open('tmp/foo', 'w') as f:
        f.write('foo\n')
    global have_symlinks
    if have_symlinks:
        try:
            os.symlink('../subdir1', 'tmp/subdir2/symlink')
            os.symlink(os.getcwd(), 'tmp/root_symlink')
            os.symlink('../foo', 'tmp/subdir1/foo_symlink')
        except:
            have_symlinks = False
    if os.path.exists(preppy):
        cmd = 'python %s 2> %s.err 1> %s.out' % (preppy, preppy, preppy)
        if os.system(cmd):
            os.system('cat %s.out' % preppy);
            os.system('cat %s.err' % preppy);
            print("prep command failed:", cmd)
            exit(1)

def compiler(exe, ccode):
    return 'x86_64-w64-mingw32-gcc -Wall -O2 -o %s %s' % (exe, ccode)

if platform != 'linux':
    runcode = lambda test,base: 'bigbro.exe %s 2> %s.err 1> %s.out' % (test, base, base)
else:
    runcode = lambda test,base: 'wine64-development bigbro.exe %s 2> %s.err 1> %s.out' % (test, base, base)

print('running C tests:')
print('================')
for testc in glob.glob('tests/*.c'):
    base = testc[:-2]
    m = importlib.import_module('tests.'+base[6:])
    if 'skip_windows' in dir(m):
        print('skipping test', base, 'not supported by windows')
        continue
    test = base+'-test.exe'
    cmd = compiler(test, testc)
    print(cmd)
    if os.system(cmd):
        print('%s fails to compile, skipping test' % (testc))
        continue
    try:
        if m.needs_symlinks and not have_symlinks:
            if flag == '':
                print('skipping', test, 'since we have no symlinks')
            continue
    except:
        print(test, 'needs to specify needs_symlinks')
        exit(1)
    create_clean_tree()
    before = perf_counter()
    cmd = runcode(test, base)
    print(cmd)
    exitcode = os.system(cmd)
    measured_time = perf_counter() - before
    err = open(base+'.err','r').read()
    out = open(base+'.out','r').read()
    m = importlib.import_module('tests.'+base[6:])
    if benchmark:
        create_clean_tree()
        before = perf_counter()
        cmd = '%s 2> %s.err 1> %s.out' % (test, base, base)
        os.system(cmd)
        reference_time = perf_counter() - before
        if measured_time < 1e-3:
            time_took = '(%g vs %g us)' % (measured_time*1e6, reference_time*1e6)
        elif measured_time < 1:
            time_took = '(%g vs %g ms)' % (measured_time*1e3, reference_time*1e3)
        else:
            time_took = '(%g vs %g s)' % (measured_time, reference_time)
    else:
        if measured_time < 1e-3:
            time_took = '(%g us)' % (measured_time*1e6)
        else:
            time_took = '(%g ms)' % (measured_time*1e3)
    if 'passes_windows' in dir(m):
        if exitcode != 0:
            os.system('cat %s.out' % base);
            os.system('cat %s.err' % base);
            print(test, "COMMAND FAILS WITH EXIT CODE", exitcode)
            numfailures += 1
        if m.passes_windows(out, err):
            print(test, "passes", time_took)
            numpasses += 1
        else:
            print(test, "FAILS!", time_took)
            numfailures += 1
    else:
        print(test, 'is not checked on windows', time_took)
        print('exit code:', exitcode)
        print('stdout:\n', out)
        print('stderr:\n', err)

test = None # to avoid bugs below where we refer to test

if numfailures > 0:
    print("\nTests FAILED (%d/%d)!!!" % (numfailures, numfailures+numpasses))
else:
    print("\nAll %d tests passed!" % numpasses)

exit(numfailures)
