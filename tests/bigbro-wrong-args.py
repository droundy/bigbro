import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/bigbro-wrong-args.sh'),
         th.reads(err, '/bigbro'),
     ])

needs_symlinks = False
