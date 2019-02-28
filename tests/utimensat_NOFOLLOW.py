import re

import tests.helper as th

def passes(out, err):
    return all(
        [th.writes(err, '/tmp/subdir1/foo_symlink'),
         th.writes(err, '/tmp/subdir1/openat'),
         th.count_writes(err, 2),
         th.count_readdir(err, 0),
     ])

needs_symlinks = False
skip_windows = True
