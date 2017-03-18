FROM debian:stable-slim

RUN apt-get -y update && apt-get -y install apt-utils gcc python3 git libc6-dev-i386 gcovr

COPY test.sh test.sh
