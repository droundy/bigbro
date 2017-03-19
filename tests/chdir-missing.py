import tests.helper as th

def passes(out, err):
    return all(
        [th.writes(err, '/tmp/still-running'),
         th.count_writes(err, 1),
         th.count_readdir(err, 0),
     ])

# def passes_windows(out, err):
#     return all(
#         [th.count_writes(err, 0),
#          th.count_readdir(err, 0),
#      ])

needs_symlinks = False
