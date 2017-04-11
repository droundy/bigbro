FROM facio/bigbro

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
