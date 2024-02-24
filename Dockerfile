# base image
FROM ubuntu:20.04
SHELL ["/bin/bash", "-c"]

# set environment
ENV HOME /master_mind_rust
# set work directory
WORKDIR ${HOME}

# install packages via apt
RUN apt update -y
RUN apt install -y tzdata
RUN apt install -y curl build-essential valgrind vim
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# copy local files into image
COPY . .
RUN source $HOME/.cargo/env && cargo build
RUN source $HOME/.cargo/env && cargo build -r
RUN valgrind --tool=callgrind --callgrind-out-file=./callgrind.out ./target/debug/master_mind_rust 5 4 --mode mktree > /dev/null
RUN valgrind --tool=massif --massif-out-file=./massif.out ./target/debug/master_mind_rust 5 4 --mode mktree > /dev/null && ms_print massif.out > massif_print.txt
RUN ./target/release/master_mind_rust --mode benchmark | awk -F, 'BEGIN {print "color_num,pin_num,duplicate,policy,min,max,ave"} NR > 1 {key=$1 FS $2; count[key]++; sum[key]+=$6; if (min[key] == "" || $6 < min[key]) min[key]=$6; if (max[key] == "" || $6 > max[key]) max[key]=$6} END {for (key in sum) {split(key, keys, FS); print keys[1], keys[2], "true", "Minmax", min[key], max[key], sum[key]/count[key]}}' OFS=, | sort -t, -k1,1n -k2,2n > benchmark.csv
