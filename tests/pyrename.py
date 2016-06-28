import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/pyrename-test.py'),
         th.writes(err, '/tmp/hello-final'),
         th.count_writes(err, 1),
     ])

def passes_windows(out, err):
    print('out:\n' + out)
    print('err:\n' + err)
    return all(
        [th.reads(err, r'\tests\pyrename-test.py'),
         th.writes(err, r'\tmp\hello-final'),
         th.count_writes(err, 1),
     ])

needs_symlinks = False
