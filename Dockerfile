# base image
FROM ubuntu:20.04

# set environment
ENV HOME /master_mind_rust
# set work directory
WORKDIR ${HOME}

# install packages via apt
RUN apt update -y
RUN apt install -y tzdata
RUN apt install -y curl build-essential valgrind
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN echo "source $HOME/.cargo/env" >> .bashrc

# copy local files into image
COPY . .
