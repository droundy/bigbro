#!/usr/bin/python3

# bigbro filetracking library
# Copyright (C) 2017 David Roundy
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

from __future__ import print_function, division

import os, glob, subprocess, importlib

def same_output(cmd):
    try:
        with open(os.devnull, 'w') as devnull:
            ver = subprocess.check_output(cmd, stderr=devnull)
            ver2 = subprocess.check_output(['./bigbro']+cmd, stderr=devnull)
        if ver != ver2:
            print('command:', cmd)
            print('alone: %s\nbigbro: %s' % (ver, ver2))
        return ver == ver2
    except:
        return None

def run_all_tests():
    num_failed = 0
    num_passed = 0
    num_skipped = 0

    alltests = glob.glob('tests/noninterference/*.py')
    alltests.remove('tests/noninterference/test.py')
    for t in alltests:
        tname = t[len('tests/noninterference/'):-3]
        try:
            m = importlib.import_module(tname)
        except:
            m = importlib.import_module('tests.noninterference.'+tname)
        result = m.run()
        if result is None:
            num_skipped += 1
            print(tname, "SKIPPED")
        elif result:
            num_passed += 1
            print(tname, "passes")
        else:
            num_failed += 1
            print(tname, "FAILS")
    return num_passed, num_skipped, num_failed

if __name__ == '__main__':
    passed, skipped, failed = run_all_tests()
    total = passed + skipped + failed

    if failed != 0:
        print('\npassed %d/%d tests' % (passed, total))
        print('skipped %d/%d tests' % (skipped, total))
        print('FAILED %d/%d tests' % (failed, total))
        exit(1)
    if skipped != 0:
        print('\npassed %d/%d tests' % (passed, total))
        print('skipped %d/%d tests' % (skipped, total))
        exit(0)
    print('\npassed all %d tests' % passed)
