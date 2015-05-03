#!/usr/bin/python3

from __future__ import print_function

import glob, os, importlib, sys

platform = sys.platform
if platform == 'linux2':
    platform = 'linux'

assert not os.system('rm -rf tests/*.test')
print('building bigbro...')
print('==================')
assert not os.system('sh build-%s.sh' % platform)

numfailures = 0

print('running tests:')
print('==============')
for testc in glob.glob('tests/*.c'):
    base = testc[:-2]
    test = base+'.test'
    if '-static' in testc:
        if os.system('${CC-gcc} -Wall -static -O2 -o %s %s' % (test, testc)):
            print('%s fails to compile, skipping test' % testc)
            continue
    else:
        if os.system('${CC-gcc} -Wall -O2 -o %s %s' % (test, testc)):
            print('%s fails to compile, skipping test' % testc)
            continue
    os.system('rm -rf tmp*')
    os.mkdir('tmp')
    os.mkdir('tmp/subdir1')
    os.mkdir('tmp/subdir1/deepdir')
    os.mkdir('tmp/subdir2')
    os.system('echo test > tmp/subdir2/test')
    os.system('echo foo > tmp/foo')
    assert not os.system('./bigbro %s 2> %s.err 1> %s.out'
                         % (test, base, base));
    err = open(base+'.err','r').read()
    out = open(base+'.out','r').read()
    m = importlib.import_module('tests.'+base[6:])
    # print(err)
    if m.passes(out, err):
        print(test, "passes")
    else:
        print(test, "FAILS!")
        numfailures += 1
    os.system('rm -rf tmp*')

if numfailures > 0:
    print("\nTests FAILED!!!")
exit(numfailures)
