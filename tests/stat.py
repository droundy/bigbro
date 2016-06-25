import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/stat.test'),
         th.reads(err, '/tmp/foo'),
         th.count_writes(err, 0),
         th.count_readdir(err, 0),
     ])

def passes_windows(out, err):
    print('out:\n', out)
    print('err:\n', err)
    return all(
        [th.reads(err, r'\tmp\foo'),
         th.count_writes(err, 0),
         th.count_readdir(err, 0),
     ])

needs_symlinks = False
