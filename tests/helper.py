# bigbro filetracking library
# Copyright (C) 2015 David Roundy
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

import re

def doesnt_read(err, file):
    fs = re.compile(r'r: /[^\n]+%s\n' % file, re.M).findall(err)
    if len(fs) > 0:
        print('  should not have read', file)
        return False
    return True

def doesnt_ls(err, file):
    fs = re.compile(r'l: /[^\n]+%s\n' % file, re.M).findall(err)
    if len(fs) > 0:
        print('  should not have readdir', file)
        return False
    return True

def reads(err, file):
    fs = re.compile(r'r: /[^\n]+%s\n' % file, re.M).findall(err)
    if len(fs) == 0:
        print('  did not read', file)
        return False
    if len(fs) > 1:
        print('  multiple reads:', file)
        return False
    return True

def writes(err, file):
    fs = re.compile(r'w: /[^\n]+%s\n' % file, re.M).findall(err)
    if len(fs) == 0:
        print('  did not write', file)
        return False
    if len(fs) > 1:
        print('  multiple writes:', file)
        return False
    return True

def readdir(err, file):
    fs = re.compile(r'l: /[^\n]+%s\n' % file, re.M).findall(err)
    if len(fs) == 0:
        print('  did not readdir', file)
        return False
    if len(fs) > 1:
        print('  multiple readdirs:', file)
        return False
    return True

def count_readdir(err, num):
    fs = re.compile(r'l: /[^\n]+\n', re.M).findall(err)
    if len(fs) != num:
        print('  did not readdir', num, fs)
        return False
    return True

def count_reads(err, num):
    fs = re.compile(r'r: /[^\n]+\n', re.M).findall(err)
    fs = [f for f in fs if 'r: /proc/' not in f]
    if len(fs) != num:
        print('  did not read', num, fs, 'read', len(fs))
        return False
    return True

def count_writes(err, num):
    fs = re.compile(r'w: /[^\n]+\n', re.M).findall(err)
    if len(fs) != num:
        print('  did not write', num, fs, 'wrote', len(fs))
        return False
    return True

