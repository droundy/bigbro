import tests.helper as th

def passes(out, err):
    return all(
        [th.writes(err, '/tmp/new-symlink'),
         th.writes(err, '/tmp/other-link'),
         th.count_writes(err, 2),
         th.count_readdir(err, 0),
     ])

needs_symlinks = True
skip_windows = True
