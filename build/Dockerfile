FROM debian:stable-slim

RUN apt-get -y update && apt-get -y install apt-utils gcc python3 git libc6-dev-i386 gcovr lcov curl ruby-sass python3-markdown make doxygen

RUN git clone git://git.kernel.org/pub/scm/devel/sparse/chrisl/sparse.git /root/sparse && cd /root/sparse && make && cp sparse /usr/bin/

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH

COPY test.sh test.sh
