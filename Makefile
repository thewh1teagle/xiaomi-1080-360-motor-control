CC=arm-linux-gnueabihf-gcc
LDIR =$(shell pwd)/lib
LIBS=$(shell ls -p ./lib  | sed 's/^/-l:/') # get libaries names from ./lib/

all: motor

motor:
	$(CC) motor.c -L$(LDIR) $(LIBS) -o motor -w
