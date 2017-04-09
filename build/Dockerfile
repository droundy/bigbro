FROM debian:stable-slim

RUN apt-get -y update && apt-get -y install apt-utils gcc python3 git libc6-dev-i386 gcovr lcov curl ruby-sass python3-markdown make doxygen mingw-w64

RUN git clone git://git.kernel.org/pub/scm/devel/sparse/chrisl/sparse.git /root/sparse && cd /root/sparse && make && cp sparse /usr/bin/

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH

RUN rustup target add x86_64-apple-darwin x86_64-pc-windows-gnu i686-pc-windows-gnu

COPY test.sh test.sh

# build this with:
# docker build -t facio/bigbro .

# test it with:
# docker run --security-opt seccomp:../docker-security.json facio/bigbro bash test.sh
