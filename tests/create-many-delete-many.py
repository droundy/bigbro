import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/create-many-delete-many.sh'),
         th.count_writes(err, 0),
         th.reads(err, '/tmp/root_symlink'),
     ])

needs_symlinks = True
