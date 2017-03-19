import tests.helper as th

def passes(out, err):
    return all(
        [th.writes(err, '/tmp/openat'),
         th.count_mkdir(err, 0),
         th.count_writes(err, 1),
         th.count_readdir(err, 0),
     ])

needs_symlinks = False
skip_windows = True
