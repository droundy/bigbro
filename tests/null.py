def passes(out, err):
    if 'tests/null.test' not in err:
        return False
    if 'libc' not in err:
        return False
    if 'w:' in err:
        return False
    if 'l:' in err:
        return False
    return True
