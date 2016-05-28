import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/kill-child.sh'),
         th.count_writes(err, 0),
         th.reads(err, '/tmp/root_symlink'),
     ])
