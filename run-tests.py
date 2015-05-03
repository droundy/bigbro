#!/usr/bin/python3

from __future__ import print_function

import glob, os, importlib

assert not os.system('rm -rf tests/*.test')
assert not os.system('fac --makefile Makefile bigbro')
assert not os.system('fac')

numfailures = 0

for test in glob.glob('tests/*.test'):
    base = test[:-5]
    os.system('rm -rf tmp*')
    os.mkdir('tmp')
    os.mkdir('tmp/subdir1')
    os.mkdir('tmp/subdir1/deepdir')
    os.mkdir('tmp/subdir2')
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
