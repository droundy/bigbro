import re

def passes(err):
    if 'tests/forks.test' not in err:
        return False
    if 'libc' not in err:
        return False
    tmpfiles = re.compile(r'w: /[^\n]+/tmp/subdir1/deepdir/forks\n', re.M).findall(err)
    if len(tmpfiles) != 1:
        return False
    written = re.compile(r'w: /[^\n]+\n', re.M).findall(err)
    if len(written) != 1:
        print('should only write one:', written)
        return False
    readdir = re.compile(r'l: /[^\n]+\n', re.M).findall(err)
    if len(readdir) != 0:
        print('should not readdir:', readdir)
        return False
    return True
