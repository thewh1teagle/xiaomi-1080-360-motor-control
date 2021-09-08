LDFLAGS=-ldl -pthread -w 
CROSSCC = arm-linux-gnueabihf-gcc

all: cross

cross: 
	$(CROSSCC) motord.c $(LDFLAGS) -o motord  

test:
	v motord/
	gcc -shared ./mocks/libdevice_kit.c -o motord/libdevice_kit.so

clean:
	$(RM) motord motor libdevice_kit.so
