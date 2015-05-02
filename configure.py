#!/usr/bin/python3

import string, os

os.system('rm -rf testing-flags');
os.mkdir('testing-flags');
with open('testing-flags/test.c', 'w') as f:
    f.write("""int main() {
  return 0;
}
""")

cc = os.getenv('CC', 'gcc')

flags = ''
for flag in ['-Wall', '-Werror', '-O2', '-std=c99', '-g', '-mtune=native']:
    if not os.system('cd testing-flags && %s %s %s -c test.c' %
                     (cc, flags, flag)):
        flags += ' ' + flag
    else:
        print('# %s cannot use flag: %s' % (cc, flag))
if len(flags) > 0:
    flags = flags[1:]
os.system('rm -rf testing-flags')

print('# cc=', repr(cc)) # in future, we should set this dynamically
print('# cflags=', repr(flags))

print("""
| %s %s -c bigbro.c
< syscalls/linux.h
< syscalls/freebsd.h
< syscalls/darwin.h

| ar rc libbigbro.a bigbro.o; ranlib libbigbro.a
> libbigbro.a
""" % (cc, flags))

print("""
| %s %s -o bigbro -L. fileaccesses.c -lbigbro
< libbigbro.a
""" % (cc, flags))

print("""
| %s %s -o nolib-bigbro fileaccesses.c bigbro.c
< syscalls/linux.h
< syscalls/freebsd.h
< syscalls/darwin.h
> nolib-bigbro
""" % (cc, flags))
