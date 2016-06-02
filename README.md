libbigbro
=========

[![Build Status](https://travis-ci.org/droundy/bigbro.svg?branch=master)](https://travis-ci.org/droundy/bigbro)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/w0uttk4ayga2f45w?svg=true)](https://ci.appveyor.com/project/droundy/bigbro)
[![Coverage Status](https://coveralls.io/repos/droundy/bigbro/badge.svg?branch=master&service=github)](https://coveralls.io/github/droundy/bigbro?branch=master)

`libbigbro` is a library that provides a single function that enables you
to run a command (i.e. `fork` and `exec` on a posix system) and track
what files it reads or modifies.

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
