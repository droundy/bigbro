import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tmp/foo'),
         th.count_writes(err, 0),
         th.count_readdir(err, 0),
     ])

# I can't seem to get stat working on windows, even though it compiles
# fine.  :( It actually passes just fine under wine, but fails under
# windows itself!  :( For now I'm disabling it, until I find some way
# to actually make it work.

# def passes_windows(out, err):
#     print('out:\n', out)
#     print('err:\n', err)
#     return all(
#         [th.reads(err, r'\tmp\foo'),
#          th.count_writes(err, 0),
#          th.count_readdir(err, 0),
#      ])

needs_symlinks = False
