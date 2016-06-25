import tests.helper as th

def passes(out, err):
    return all(
        [th.reads(err, '/tests/null.test'),
         th.count_writes(err, 0),
         th.count_readdir(err, 0),
     ])

def passes_windows(out, err):
    '''Some day I want this to register

    th.reads(err, r'\tests\null-test.exe'),

    but for now we aren't tracking the creation of processes.
    '''
    return all(
        [th.count_writes(err, 0),
         th.count_readdir(err, 0),
     ])

needs_symlinks = False
