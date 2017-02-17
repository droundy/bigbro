import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/mkdir-and-rename.test'),
         th.writes(err, '/tmp/subdirnew/subfile'),
         th.writes(err, '/tmp/useless'),
         th.mkdirs(err, '/tmp/subdirnew'),
         th.mkdirs(err, '/tmp/subdirnew/subsub'),
         th.count_mkdir(err, 2),
         th.count_writes(err, 2),
         th.count_readdir(err, 0),
     ])

needs_symlinks = False
skip_windows = True
