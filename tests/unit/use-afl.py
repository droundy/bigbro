#!/usr/bin/python3

import subprocess, glob, os

usage = 'usage: CC=.../afl-gcc FUZZ=.../afl-fuzz python3 tests/unit/use-afl.py'

if 'CC' in os.environ:
    cc = os.environ['CC']
    print('CC is', cc)
else:
    print(usage)
    exit(1)
if 'FUZZ' in os.environ:
    fuzz = os.environ['FUZZ']
    print('FUZZ is', fuzz)
else:
    print(usage)
    exit(1)

os.environ['AFL_SKIP_CPUFREQ'] = ''

for c in glob.glob('tests/unit/*.c'):
    test = c[:-1]+'test'
    inputs = c[:-1]+'inputs'
    outputs = c[:-1]+'outputs'
    cmd = [cc, '-I.', '--std=c99', '-g', '-O2',
           '-o', test, c]
    print(' '.join(cmd))
    assert not subprocess.call(cmd)
    cmd = [fuzz, '-i', inputs, '-o', outputs, test]
    assert not subprocess.call(cmd)
