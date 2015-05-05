#!/bin/sh

echo compiler: ${CC-gcc}
echo ar: ${AR-ar}
echo ranlib: ${RANLIB-ranlib}

set -ev

python3 syscalls/darwin.py > syscalls/darwin.h
python3 syscalls/freebsd.py > syscalls/freebsd.h
python3 syscalls/linux.py > syscalls/linux.h

${CC-gcc} -Wall -std=c99 -g -O2 -c bigbro-linux.c
${CC-gcc} -Wall -std=c99 -g -O2 -c bigbro-better.c
rm -f libbigbro.a
${AR-ar} rc libbigbro.a bigbro-linux.o && ${RANLIB-ranlib} libbigbro.a

${CC-gcc} -Wall -std=c99 -g -O2 -o bigbro -L. fileaccesses.c -lbigbro

