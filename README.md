[![Windows Build Status](https://ci.appveyor.com/api/projects/status/w0uttk4ayga2f45w?svg=true)](https://ci.appveyor.com/project/droundy/bigbro)
[![CircleCI](https://circleci.com/gh/droundy/bigbro.svg?style=svg)](https://circleci.com/gh/droundy/bigbro)
<!-- [![Build Status](https://travis-ci.org/droundy/bigbro.svg?branch=master)](https://travis-ci.org/droundy/bigbro) -->

[![Coverage Status](https://coveralls.io/repos/droundy/bigbro/badge.svg?branch=master&service=github)](https://coveralls.io/github/droundy/bigbro?branch=master)
[![codecov](https://codecov.io/gl/facio/bigbro/branch/master/graph/badge.svg)](https://codecov.io/gl/facio/bigbro)
[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/203/badge)](https://bestpractices.coreinfrastructure.org/projects/203)


libbigbro
=========

`libbigbro` is a library that provides two functions that enables you
to run a command (i.e. `fork` and `exec` on a posix system) and track
what files it reads or modifies.  These two functions, `bigbro` and
`bigbro_with_mkdir` are declared in `bigbro.h`, which documents their
behavior in a comment.

bigbro
------

`bigbro` is a simple demo utility that uses `libbigbro` to run a
changes specified on the command line.  It has no command-line flags,
and is as easy to use as `strace`:


    $ ./bigbro mail
    No mail for droundy
    r: /usr/lib/x86_64-linux-gnu/liblockfile.so.1.0
    r: /usr/lib/x86_64-linux-gnu/liblockfile.so.1
    r: /usr/bin/bsd-mailx
    r: /usr/bin/mail
    r: /lib/x86_64-linux-gnu/libc.so.6
    r: /lib/x86_64-linux-gnu/libbsd.so.0.7.0
    r: /lib/x86_64-linux-gnu/libc-2.19.so
    r: /lib/x86_64-linux-gnu/libbsd.so.0
    r: /etc/mail.rc
    r: /etc/ld.so.cache
    r: /etc/alternatives/mail

The purpose of `bigbro` is primarily to enable easy testing of
`libbigbro`.

Building
--------

To build bigbro, you just need run:

    sh build/linux.sh

If you have [fac](http://physics.oregonstate.edu/~roundyd/fac)
installed, you can alternatively build bigbro using fac.  In this
case, you can also simultaneously build bigbro and run the test suite,
by running:

    python3 run-tests.py
