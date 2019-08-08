## -*- docker-image-name: "clojureclibuild" -*-

# docker run -i -t clojureclibuild /bin/bash
# 

# Use an official ubuntu runtime as a parent image
FROM rust:latest


# make ready to compile nsis
RUN apt-get update -y
RUN apt -y install python2.7 python-pip scons
RUN mkdir /nsis && \
	curl -L https://netix.dl.sourceforge.net/project/nsis/NSIS%203/3.04/nsis-3.04.zip -o /nsis/nsis-3.04.zip && \
	curl -L https://netix.dl.sourceforge.net/project/nsis/NSIS%203/3.04/nsis-3.04-src.tar.bz2 -o /nsis/nsis-3.04-src.tar.bz2 && \
	unzip /nsis/nsis-3.04.zip -d /nsis && \
	tar -C /nsis -xvjf /nsis/nsis-3.04-src.tar.bz2

# compile nsis
RUN cd /nsis/nsis-3.04-src && \
	scons SKIPSTUBS=all SKIPPLUGINS=all SKIPUTILS=all SKIPMISC=all NSIS_CONFIG_CONST_DATA=no PREFIX=/nsis/nsis-3.04 install-compiler

RUN chmod +x /nsis/nsis-3.04/bin/makensis && \
	ln -s /nsis/nsis-3.04/bin/makensis /usr/local/bin/makensis && \
	mkdir /nsis/nsis-3.04/share && \
	ln -s /nsis/nsis-3.04 /nsis/nsis-3.04/share/nsis

# upx
RUN apt-get update -y
RUN apt-get install -y upx zip
RUN apt-get install gcc-mingw-w64-x86-64 -y
RUN rustup target add x86_64-pc-windows-gnu

# cd ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/
# mv crt2.o crt2.o.bak
# cp /usr/x86_64-w64-mingw32/lib/crt2.o ./

# our project
WORKDIR /portable-clojure-cli-rust

COPY . .

# /usr/local/rustup/toolchains/1.36.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/crt2.o
