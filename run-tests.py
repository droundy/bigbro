#!/usr/bin/python3

from __future__ import print_function

import glob, os, importlib

assert not os.system('fac --makefile Makefile bigbro')
assert not os.system('fac')

for test in glob.glob('tests/*.test'):
    base = test[:-5]
    assert not os.system('./bigbro %s 2> %s.err 1> %s.out'
                         % (test, base, base));
    err = open(base+'.err','r').read()
    m = importlib.import_module('tests.'+base[6:])
    # print(err)
    if m.passes(err):
        print(test, "PASSES!")
    else:
        print(test, "FAILS!")
    os.system('rm -rf tmp*')
