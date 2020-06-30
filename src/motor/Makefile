LDFLAGS=-ldl -pthread -w 
CC = gcc
CROSSCC = arm-linux-gnueabihf-gcc

all: cross

cross: 
	$(CROSSCC) motord.c $(LDFLAGS) -o motord  

test:
	$(CC) motord.c $(LDFLAGS) -o motord  
	$(CC) -shared ./mocks/libdevice_kit.c -o libdevice_kit.so

clean:
	$(RM) motord libdevice_kit.so event status
