#!/usr/bin/python3

from __future__ import print_function

import glob, os, subprocess, importlib, sys, time, shutil

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

print('building bigbro by running build/windows.py...')
print('============================================')

assert not os.system('python3 build/windows.py')

numfailures = 0

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
        cmd = 'python3 %s 2> %s.err 1> %s.out' % (preppy, preppy, preppy)
        if os.system(cmd):
            os.system('cat %s.out' % preppy);
            os.system('cat %s.err' % preppy);
            print("prep command failed:", cmd)
            exit(1)

for compiler in ['cl', 'x86_64-w64-mingw32-gcc', 'cc']:
    try:
        subprocess.call([compiler, '--version'])
        cc = compiler
        print('using',cc,'compiler')
        break
    except:
        print('NOT using',compiler,'compiler')

print('running C tests:')
print('================')
for testc in glob.glob('tests/*.c'):
    base = testc[:-2]
    m = importlib.import_module('tests.'+base[6:])
    if 'skip_windows' in dir(m):
        print('skipping test', base, 'not supported by windows')
        continue
    test = base+'-test.exe'
    cmd = '%s -Wall -O2 -o %s %s' % (compiler, test, testc)
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
    if compiler == 'cl':
        cmd = 'bigbro.exe %s 2> %s.err 1> %s.out' % (test, base, base)
    else:
        cmd = 'wine64 bigbro.exe %s 2> %s.err 1> %s.out' % (test, base, base)
    if os.system(cmd):
        os.system('cat %s.out' % base);
        os.system('cat %s.err' % base);
        print("command failed:", cmd)
        exit(1)
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
        if m.passes_windows(out, err):
            print(test, "passes", time_took)
        else:
            print(test, "FAILS!", time_took)
            numfailures += 1
    else:
        print(test, 'is not checked on windows', time_took)

test = None # to avoid bugs below where we refer to test

if numfailures > 0:
    print("\nTests FAILED!!!")
else:
    print("\nAll tests passed!")

exit(numfailures)
