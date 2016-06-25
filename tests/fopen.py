import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/fopen.test'),
         th.writes(err, '/tmp/foo'),
         th.count_writes(err, 1),
         th.count_readdir(err, 0),
     ])

def passes_windows(out, err):
    return all(
        [th.writes(err, r'\tmp\foo'),
         th.count_writes(err, 1),
         th.count_readdir(err, 0),
     ])

needs_symlinks = False
