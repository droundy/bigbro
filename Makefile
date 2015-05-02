all: bigbro

syscalls/darwin.h : syscalls syscalls/darwin.py syscalls/darwin/syscalls.master syscalls/gentables.py syscalls/master.py
	python3 syscalls/darwin.py > syscalls/darwin.h

syscalls/freebsd.h : syscalls syscalls/freebsd.py syscalls/freebsd/syscalls.master syscalls/gentables.py syscalls/master.py
	python3 syscalls/freebsd.py > syscalls/freebsd.h

syscalls/linux.h : syscalls syscalls/gentables.py syscalls/linux.py syscalls/linux/unistd_32.h syscalls/linux/unistd_64.h
	python3 syscalls/linux.py > syscalls/linux.h

bigbro.o : bigbro.c bigbro.h errors.h intmap.c intmap.h iterablehash.c iterablehash.h posixmodel.c posixmodel.h syscalls/darwin.h syscalls/freebsd.h syscalls/linux.h
	gcc -Wall -Werror -O2 -std=c99 -g -mtune=native -c bigbro.c

libbigbro.a : bigbro.o
	ar rc libbigbro.a bigbro.o; ranlib libbigbro.a

bigbro : bigbro.h fileaccesses.c libbigbro.a
	gcc -Wall -Werror -O2 -std=c99 -g -mtune=native -o bigbro -L. fileaccesses.c -lbigbro

