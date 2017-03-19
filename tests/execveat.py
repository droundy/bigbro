import re

import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/linkat.test'),
         th.reads(err, '/tests/null.test'),
         th.writes(err, '/tmp/subdir2/hidden/awesome'),
         th.mkdirs(err, '/tmp/subdir2/hidden'),
         th.count_writes(err, 1),
         th.count_readdir(err, 0),
     ])

needs_symlinks = False
skip_windows = True
