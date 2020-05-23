CC=arm-linux-gnueabihf-gcc
CFLAGS = -Wall
CFILES := $(wildcard *.c)
ODIR =$(shell pwd)/lib
OBJS=$(shell ls -p ./lib  | sed 's/^/-l:/') # get libaries names from ./lib/

all:
	
	
	$(CC) $(CFLAGS) $(CFILES) -L$(ODIR) $(OBJS) -o motor -w

clean:
	rm -rf motor

