import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/truncate.sh'),
         th.count_writes(err, 2),
         th.reads(err, '/tmp/root_symlink'),
         th.writes(err, '/tmp/foo'),
         th.writes(err, '/tmp/foobar'),
     ])
