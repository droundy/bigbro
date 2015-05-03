all: bigbro

syscalls/darwin.h : syscalls/darwin.py syscalls/darwin/syscalls.master syscalls/gentables.py syscalls/master.py
	python3 syscalls/darwin.py > syscalls/darwin.h

syscalls/freebsd.h : syscalls/freebsd.py syscalls/freebsd/syscalls.master syscalls/gentables.py syscalls/master.py
	python3 syscalls/freebsd.py > syscalls/freebsd.h

syscalls/linux.h : syscalls/gentables.py syscalls/linux.py syscalls/linux/unistd_32.h syscalls/linux/unistd_64.h
	python3 syscalls/linux.py > syscalls/linux.h

bigbro-linux.o : bigbro-linux.c bigbro.h errors.h intmap.c intmap.h iterablehash.c iterablehash.h posixmodel.c posixmodel.h syscalls/darwin.h syscalls/freebsd.h syscalls/linux.h
	gcc -Wall -Werror -O2 -std=c99 -g -mtune=native -c bigbro-linux.c

libbigbro.a : bigbro-linux.o
	ar rc libbigbro.a bigbro-linux.o && ranlib libbigbro.a

bigbro : bigbro.h fileaccesses.c libbigbro.a
	gcc -Wall -Werror -O2 -std=c99 -g -mtune=native -o bigbro -L. fileaccesses.c -lbigbro

