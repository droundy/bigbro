import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/rename.test'),
         th.writes(err, '/tmp/barbaz'),
         th.count_writes(err, 1),
         th.count_readdir(err, 0),
     ])
