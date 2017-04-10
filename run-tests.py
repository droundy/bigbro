#!/usr/bin/python3

# bigbro filetracking library
# Copyright (C) 2015,2016,2017 David Roundy
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

import glob, os, importlib, sys, time, shutil

import tests.noninterference.test as noninterference

if sys.version_info < (3,2):
    print('Please run this script with python 3.2 or newer.', sys.version_info)
    exit(1)

# The following code disables caching on stdout.  It's a bit hacky,
# but really pays of when the tests are taking a long time running
# under docker, and you can't tell whether it is frozen.
class Unbuffered(object):
   def __init__(self, stream):
       self.stream = stream
   def write(self, data):
       self.stream.write(data)
       self.stream.flush()
   def __getattr__(self, attr):
       return getattr(self.stream, attr)
sys.stdout = Unbuffered(sys.stdout)

if 'perf_counter' in dir(time):
    perf_counter = time.perf_counter
else:
    perf_counter = time.time
def format_times(measured_time, reference_time=-1):
    if reference_time > 0:
        if measured_time < 1e-3:
            return '(%g vs %g us)' % (measured_time*1e6, reference_time*1e6)
        elif measured_time < 1:
            return '(%g vs %g ms)' % (measured_time*1e3, reference_time*1e3)
        else:
            return '(%g vs %g s)' % (measured_time, reference_time)
    if measured_time < 1e-3:
        return '(%g us)' % (measured_time*1e6)
    elif measured_time < 1:
        return '(%g ms)' % (measured_time*1e3)
    else:
        return '(%g s)' % (measured_time)

benchmark = 'bench' in sys.argv

platform = sys.platform
if platform == 'linux2':
    platform = 'linux'

for f in glob.glob('tests/*.test') + glob.glob('*.gcno') + glob.glob('*.gcda'):
    os.remove(f)

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

# we always run with test coverage if lcov is present!
have_lcov = not benchmark and is_in_path('lcov')
have_gcovr = not benchmark and is_in_path('gcovr')

print('creating build/%s.sh...' % platform)
print('==========================')
if not os.system('fac --help'):
    assert not os.system('fac --script build/%s.sh libbigbro.a bigbro' % platform)
if not os.system('fac --help'):
    os.system('fac --script build/cross-windows.sh bigbro.exe bigbro32.dll bigbro64.dll')

print('building bigbro by running build/%s.sh...' % platform)
print('============================================')

if not os.system('fac --help'):
    assert not os.system('fac')

if have_lcov or have_gcovr:
    os.environ['CFLAGS'] = os.environ.get('CFLAGS', default='') + ' --coverage'
    os.environ['LDFLAGS'] = os.environ.get('LDFLAGS', default='') + ' --coverage'

if not os.system('fac --help'):
    assert not os.system('fac bigbro')

assert not os.system('sh build/%s.sh' % platform)

if have_lcov:
    assert not os.system('lcov --config-file .lcovrc -c -i -d . -o tests/base.info')

numfailures = 0
numpasses = 0

have_symlinks = True

def should_fail(m):
    try:
        if m.should_fail:
            return True
    except:
        return False
    return False

def create_clean_tree(prepsh='this file does not exist'):
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
    if os.path.exists(prepsh):
        cmd = 'sh %s 2> %s.err 1> %s.out' % (prepsh, prepsh, prepsh)
        if os.system(cmd):
            print('the command')
            os.system('cat ' + prepsh)
            print('stdout')
            os.system('cat %s.out' % prepsh);
            print('stderr')
            os.system('cat %s.err' % prepsh);
            print("prep command failed:", cmd)
            print('rerunning sh ' + prepsh)
            os.system('sh '+prepsh)
            print("reiterating that prep command failed:", cmd)
            exit(1)

print('running AFL tests:')
print('==================')
for testc in glob.glob('tests/unit/*.c'):
    base = testc[:-2]
    test = base+'.test'
    if os.system('${CC-gcc} --coverage -I. --std=c99 -Wall -O2 -o %s %s' % (test, testc)):
        print('%s fails to compile, skipping unit test' % (testc))
        continue
    for inp in glob.glob('%s.minimal/*' % base):
        cmd = '%s < %s > /dev/null' % (test, inp)
        exitcode = os.system(cmd)
        if exitcode != 0:
            print(test[len('tests/unit/'):], '<', inp[len(base+'.minimal/'):],
                  "FAILS WITH EXIT CODE", exitcode)
            numfailures += 1
        else:
            print(test[len('tests/unit/'):], '<', inp[len(base+'.minimal/'):], "passes")
            numpasses += 1

options = ['', '-m32', '-m64'] # , ' -mx32']

print('compiling C tests:')
print('================')
ctest_executables = []
for testc in glob.glob('tests/*.c'):
    base = testc[:-2]
    for flag in options:
        test = base + flag + '.test'
        if '-static' in testc:
            if os.system('${CC-gcc} %s -Wall -std=c99 -static -O2 -o %s %s' % (flag, test, testc)):
                print('%s %s fails to compile, skipping test' % (testc, flag))
                continue
        else:
            if os.system('${CC-gcc} %s -Wall -std=c99 -O2 -o %s %s' % (flag, test, testc)):
                print('%s %s fails to compile, skipping test' % (testc, flag))
                continue
        ctest_executables.append((base,test))

bigbro_binaries = ['./bigbro']
if is_in_path('cargo'):
    bigbro_binaries += ['target/debug/test-bigbro', 'target/release/test-bigbro']

for bigbro in bigbro_binaries:
    print('running C tests with %s:' % bigbro)
    print('================')
    for (base,test) in ctest_executables:
        m = importlib.import_module('tests.'+base[6:])
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
        cmd = '%s %s 2> %s.err 1> %s.out' % (bigbro, test, base, base)
        exitcode = os.system(cmd)
        measured_time = perf_counter() - before
        err = open(base+'.err','r').read()
        out = open(base+'.out','r').read()
        # print(err)
        if benchmark:
            create_clean_tree()
            before = perf_counter()
            cmd = '%s 2> %s.err 1> %s.out' % (test, base, base)
            os.system(cmd)
            reference_time = perf_counter() - before
            time_took = format_times(measured_time, reference_time)
        else:
            time_took = format_times(measured_time)
        if exitcode != 0:
            os.system('cat %s.out' % base);
            os.system('cat %s.err' % base);
            print(test, "COMMAND FAILS WITH EXIT CODE", exitcode)
            numfailures += 1
        elif m.passes(out, err):
            print(test, "passes", time_took)
            numpasses += 1
        else:
            print(test, "FAILS!", time_took)
            numfailures += 1

    test = None # to avoid bugs below where we refer to test
    print()

for bigbro in bigbro_binaries:
    print('running sh tests with %s:' % bigbro)
    print('=================')
    for testsh in glob.glob('tests/*.sh'):
        base = testsh[:-3]
        m = importlib.import_module('tests.'+base[6:])
        try:
            if m.needs_symlinks and not have_symlinks:
                print('skipping', test, 'since we have no symlinks')
                continue
        except:
            print(test, 'needs to specify needs_symlinks')
            exit(1)
        create_clean_tree(testsh+'.prep')
        before = perf_counter()
        cmd = '%s sh %s 2> %s.err 1> %s.out' % (bigbro, testsh, base, base)
        if os.system(cmd) and not should_fail(m):
            os.system('cat %s.out' % base);
            os.system('cat %s.err' % base);
            print("FAIL command failed:", cmd)
            numfailures += 1
            os.system('%s sh %s' % (bigbro, testsh))
            continue
        measured_time = perf_counter() - before
        err = open(base+'.err','r').read()
        out = open(base+'.out','r').read()
        if benchmark:
            create_clean_tree(testsh+'.prep')
            before = perf_counter()
            cmd = 'sh %s 2> %s.err 1> %s.out' % (testsh, base, base)
            os.system(cmd)
            reference_time = perf_counter() - before
            time_took = format_times(measured_time, reference_time)
        else:
            time_took = format_times(measured_time)
        # print(err)
        if m.passes(out, err):
            print(testsh, "passes", time_took)
            numpasses += 1
        else:
            print(testsh, "FAILS!", time_took)
            numfailures += 1
    print()

for bigbro in bigbro_binaries:
    print('running python tests with %s:' % bigbro)
    print('=====================')
    for testp in glob.glob('tests/*-test.py'):
        base = testp[:-8]
        m = importlib.import_module('tests.'+base[6:])
        try:
            if m.needs_symlinks and not have_symlinks:
                print('skipping', test, 'since we have no symlinks')
                continue
        except:
            print(test, 'needs to specify needs_symlinks')
            exit(1)
        create_clean_tree(testp+'.prep')
        before = perf_counter()
        cmd = '%s python %s 2> %s.err 1> %s.out' % (bigbro, testp, base, base)
        if os.system(cmd):
            os.system('cat %s.out' % base);
            os.system('cat %s.err' % base);
            print("command failed:", cmd)
            exit(1)
        measured_time = perf_counter() - before
        err = open(base+'.err','r').read()
        out = open(base+'.out','r').read()
        if benchmark:
            create_clean_tree(testp+'.prep')
            before = perf_counter()
            cmd = 'sh %s 2> %s.err 1> %s.out' % (testp, base, base)
            os.system(cmd)
            reference_time = perf_counter() - before
            time_took = format_times(measured_time, reference_time)
        else:
            time_took = format_times(measured_time)
        # print(err)
        if m.passes(out, err):
            print(testp, "passes", time_took)
            numpasses += 1
        else:
            print(testp, "FAILS!", time_took)
            numfailures += 1
    print()

for bigbro in bigbro_binaries:
    if benchmark:
        print()
        print('running sh benchmarks with %s:' % bigbro)
        print('======================')
        for testsh in glob.glob('bench/*.sh'):
            base = testsh[:-3]
            create_clean_tree(testsh+'.prep')
            before = perf_counter()
            cmd = '%s sh %s 2> %s.err 1> %s.out' % (bigbro, testsh, base, base)
            if os.system(cmd):
                os.system('cat %s.out' % base);
                os.system('cat %s.err' % base);
                print("command failed:", cmd)
                exit(1)
            measured_time = perf_counter() - before
            # The first time is just to warm up the file system cache...

            create_clean_tree(testsh+'.prep')
            before = perf_counter()
            cmd = 'sh %s 2> %s.err 1> %s.out' % (testsh, base, base)
            os.system(cmd)
            reference_time = perf_counter() - before
            before = perf_counter()
            cmd = '%s sh %s 2> %s.err 1> %s.out' % (bigbro, testsh, base, base)
            os.system(cmd)
            measured_time = perf_counter() - before
            time_took = format_times(measured_time, reference_time)
            print(testsh, time_took)

if have_lcov:
    assert not os.system('lcov --config-file .lcovrc -c -d . -o tests/test.info')
    assert not os.system('lcov --config-file .lcovrc -a tests/base.info -a tests/test.info -o tests/coverage.info')
    assert not os.system('lcov --config-file .lcovrc --remove tests/coverage.info "/usr/*" --output-file tests/coverage.info')
    assert not os.system('rm -rf web/coverage')
    assert not os.system('genhtml --config-file .lcovrc --show-details -o web/coverage -t "bigbro coverage" tests/coverage.info')

if have_gcovr:
    assert not os.system('gcovr -k -r . --exclude-unreachable-branches --html --html-details -o web/coverage.html')
    assert not os.system('gcovr -r . --exclude-unreachable-branches')

p,s,f = noninterference.run_all_tests()
numfailures += f
numpasses += p

if numfailures > 0:
    print("\nTests FAILED (%d)!!!" % numfailures)
else:
    print("\nAll %d tests passed!" % numpasses)

exit(numfailures)
