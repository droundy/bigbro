#!/bin/sh

set -ev

(python3 syscalls/linux.py > syscalls/linux.h)

(gcc  -O2 -Wall -Werror -std=c99 -g -mtune=native -c bigbro-linux.c)

(rm -f libbigbro.a && ${AR-ar} rc libbigbro.a bigbro-linux.o && ${RANLIB-ranlib} libbigbro.a)

(gcc  -O2 -Wall -Werror -std=c99 -g -mtune=native -o bigbro -L. fileaccesses.c -lbigbro)

