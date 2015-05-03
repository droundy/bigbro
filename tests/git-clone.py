import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/git-clone.sh'),
         th.writes(err, '/tmp/subdir1/bigbro/README.md'),
         th.writes(err, '/tmp/subdir1/bigbro/bigbro-linux.c'),
     ])
