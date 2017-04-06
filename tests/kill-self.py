import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/kill-self.sh'),
         th.writes(err, 'tmp/foo'),
         th.count_writes(err, 1),
         th.reads(err, '/tmp/root_symlink'),
     ])

needs_symlinks = True

should_fail = True
