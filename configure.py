#!/usr/bin/python3

import string, os, glob, sys

os.system('rm -rf testing-flags');
os.mkdir('testing-flags');
with open('testing-flags/test.c', 'w') as f:
    f.write("""int main() {
  return 0;
}
""")

cc = os.getenv('CC', 'gcc')

cflags = ''
for flag in ['-Wall', '-Werror', '-O2', '-std=c99', '-g', '-mtune=native']:
    if not os.system('cd testing-flags && %s %s %s -c test.c' %
                     (cc, cflags, flag)):
        cflags += ' ' + flag
    else:
        print('# %s cannot use flag: %s' % (cc, flag))
if len(cflags) > 0:
    cflags = cflags[1:]
os.system('rm -rf testing-flags')

print('# cc=', repr(cc)) # in future, we should set this dynamically
print('# cflags=', repr(cflags))

print("""
| %s %s -c bigbro-%s.c
< syscalls/linux.h
< syscalls/freebsd.h
< syscalls/darwin.h

| ar rc libbigbro.a bigbro-%s.o; ranlib libbigbro.a
> libbigbro.a
""" % (cc, cflags, sys.platform, sys.platform))

print("""
| %s %s -o bigbro -L. fileaccesses.c -lbigbro
< libbigbro.a
""" % (cc, cflags))

print("""
| %s %s -o nolib-bigbro fileaccesses.c bigbro-%s.c
< syscalls/linux.h
< syscalls/freebsd.h
< syscalls/darwin.h
> nolib-bigbro
""" % (cc, cflags, sys.platform))

for testc in glob.glob('tests/*.c'):
    base = testc[:-2]
    print("""
| %s %s -o %s.test %s
    """ % (cc, cflags, base, testc))
