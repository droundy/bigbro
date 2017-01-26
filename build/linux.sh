#!/bin/sh

set -ev

(python3 syscalls/linux.py > syscalls/linux.h)

(gcc $CFLAGS -O2 -Wall -Werror -std=c99 -g -mtune=native -fpic -c bigbro-linux.c)

(rm -f libbigbro.a && ${AR-ar} rc libbigbro.a bigbro-linux.o && ${RANLIB-ranlib} libbigbro.a)

(gcc $CFLAGS -O2 -Wall -Werror -std=c99 -g -mtune=native -fpic -static -o bigbro -L. fileaccesses.c -lbigbro)

