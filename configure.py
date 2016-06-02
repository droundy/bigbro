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

import string, os, glob, sys

platform = sys.platform
if platform == 'linux2':
    platform = 'linux'

os.system('rm -rf testing-flags');
os.mkdir('testing-flags');
with open('testing-flags/test.c', 'w') as f:
    f.write("""int main() {
  return 0;
}
""")

cc = '${CC-gcc}'

cflags = '${CFLAGS--O2}'
for flag in ['-Wall', '-Werror', '-std=c99', '-g', '-mtune=native']:
    if not os.system('cd testing-flags && %s %s %s -c test.c' %
                     (cc, cflags, flag)):
        cflags += ' ' + flag
    else:
        print('# %s cannot use flag: %s' % (cc, flag))
os.system('rm -rf testing-flags')

print('# cc=', repr(cc)) # in future, we should set this dynamically
print('# cflags=', repr(cflags))

print("""
| %s %s -c bigbro-%s.c
< syscalls/linux.h
> bigbro-%s.o

# We need to remove libbigbro.a before running ar, because otherwise
# it will be added to, rather than replaced.

| rm -f libbigbro.a && ${AR-ar} rc libbigbro.a bigbro-%s.o && ${RANLIB-ranlib} libbigbro.a
< bigbro-%s.o
> libbigbro.a
""" % (cc, cflags, platform, platform, platform, platform))

print("""
| %s %s -o bigbro -L. fileaccesses.c -lbigbro
< libbigbro.a
> bigbro
""" % (cc, cflags))
