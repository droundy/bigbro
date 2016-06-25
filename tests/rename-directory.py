import tests.helper as th

def passes(out, err):
    """This test illustrates a limitation of how we handle directory
    renames: we only track changes to files within that directory that
    we have written to or read from.  This means that you cannot take
    the output of two sequential jobs and use it to find the net
    change, since the second job may rename files written to in the
    first job.  In the context of fac, I don't see this being a
    problem.  A complete fix would involve tracking deletions as well
    as writes.

    """
    return all(
        [th.reads(err, '/tests/rename-directory.test'),
         th.count_writes(err, 2),
         th.doesnt_write(err, '/tmp/subdir2/hello'),
         th.doesnt_read(err, '/tmp/subdir2/test'),
         th.writes(err, '/tmp/newdir/hello'),
         th.writes(err, '/tmp/newdir/test'),
     ])

needs_symlinks = True
