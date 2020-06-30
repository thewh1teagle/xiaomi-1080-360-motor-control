# docker build -t name .

# docker run -v -it /path/on/host:/data container-image



FROM ubuntu:18.04


RUN apt-get update && apt-get install -y  \
	bash-completion vim nano git curl wget unzip \
	g++-arm-linux-gnueabihf make file tmux \
	gcc

RUN mkdir /src

WORKDIR /src
