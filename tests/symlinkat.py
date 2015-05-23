import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/symlinkat.test'),
         th.writes(err, '/tmp/new-symlink'),
         th.count_writes(err, 1),
         th.count_readdir(err, 0),
     ])
