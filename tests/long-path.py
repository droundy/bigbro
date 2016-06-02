import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/long-path.sh'),
         th.count_writes(err, 80),
         th.reads(err, '/tmp/root_symlink'),
         th.writes(err, 'tmp/dirfoo/dirfoo/dirfoo/dir foo/dir  foo/dir   foo/dir    foo/dir     foo/dir      foo/dir       foo/dir        foo/dir         foo/dir          foo/dir           foo/dir            foo/dir             foo/dir              foo/dir               foo/dir                foo/dir                 foo/dir                  foo/dir                   foo/dir                    foo/dir                     foo/dir                      foo/dir                       foo/dir                        foo/dir                         foo/dir                          foo/dir                           foo/dir                            foo/dir                             foo/dir                              foo/dir                               foo/file                               .dat'),
     ])

needs_symlinks = True
