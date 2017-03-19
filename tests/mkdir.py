import tests.helper as th

def passes(out, err):
    return all(
        [th.mkdirs(err, '/tmp/subdirnew'),
         th.count_mkdir(err, 1),
         th.count_writes(err, 0),
         th.count_readdir(err, 0),
     ])

needs_symlinks = False
skip_windows = True
