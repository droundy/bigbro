import tests.helper as th

def passes(out, err):
    return all(
        [th.writes(err, '/tmp/root_symlink'),
         th.count_writes(err, 1),
         th.count_readdir(err, 0),
     ])

needs_symlinks = True
