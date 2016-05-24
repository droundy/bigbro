#!/usr/bin/python3

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

cflags = '${CFLAGS-}'
for flag in ['-Wall', '-Werror', '-O2', '-std=c99', '-g', '-mtune=native']:
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
