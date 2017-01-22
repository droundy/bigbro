import re

def passes(out, err):
    if 'tests/mkdir.test' not in err:
        return False
    if 'libc' not in err:
        return False
    tmpfiles = re.compile(r'm: /[^\n]+/tmp/subdirnew\n', re.M).findall(err)
    if len(tmpfiles) != 1:
        return False
    written = re.compile(r'm: /[^\n]+\n', re.M).findall(err)
    if len(written) != 1:
        print('should only mkdir one:', written)
        return False
    readdir = re.compile(r'l: /[^\n]+\n', re.M).findall(err)
    if len(readdir) != 0:
        print('should not readdir:', readdir)
        return False
    return True

needs_symlinks = False
skip_windows = True
