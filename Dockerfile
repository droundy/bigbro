FROM debian:stable-slim

# I use separate RUN statements here hoping to be able to make changes
#  without invalidating the entire cache.

RUN apt-get -y update
RUN apt-get -y install apt-utils
RUN apt-get -y install gcc python3 git libc6-dev-i386
RUN apt-get -y install lcov
RUN apt-get -y install gcovr

RUN useradd developer
RUN mkdir /home/developer
RUN chown developer:developer /home/developer

# COPY . /home/developer/git-source-repo
# RUN su developer -c 'git clone /home/developer/git-source-repo /home/developer/bigbro'
COPY . /home/developer/bigbro
RUN chown -R developer:developer /home/developer/bigbro

WORKDIR /home/developer/bigbro
USER developer

# execute this with:
# docker build -t bigbro . && docker run --security-opt seccomp:./docker-security.json bigbro python3 run-tests.py
