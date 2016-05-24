#!/usr/bin/python2

import re, os

re_syscall = re.compile(r'^([0-9]+)\s+\S+\s+(\S+)')

sysnames = {}
allsysnames = []

for postfix in ['_32', '_64']:
    if os.uname()[4] == 'x86_64':
        systbl = "syscalls/linux/syscall%s.tbl" % postfix
    else:
        systbl = "syscalls/linux/syscall_32.tbl"

    with open(systbl, 'r') as table_file:
        table = table_file.read()

    sysnames[postfix] = {}
    for line in table.split('\n'):
        m = re_syscall.match(line)
        if m != None:
            num, name = m.groups()
            num = int(num)
            #print 'got', num, name
            sysnames[postfix][num] = name
            if name not in allsysnames:
                allsysnames.append(name)
        else:
            # print('# failed match at', line)
            pass

print("""
enum syscall {""")
for i in range(len(allsysnames)-1):
    print('  sc_%s = %d,' % (allsysnames[i], i))
print('  sc_%s' % allsysnames[-1])
print("};\n")

print("""
const char *syscall_names[] = {""")
for i in range(len(allsysnames)-1):
    print('  "%s",' % (allsysnames[i]))
print('  "%s"' % allsysnames[-1])
print("};\n")

for postfix in ['_32', '_64']:
    maxnum = len(sysnames[postfix])-1
    print("""
static inline enum syscall syscalls%s(int num) {
   switch (num) {""" % postfix)
    for i in sysnames[postfix].keys():
        print('    case %d: return sc_%s;' % (i, sysnames[postfix][i]))
    print("""default: return -1;
    }
};\n""")
