LDFLAGS=-ldl -pthread -w 

ifeq ($(TARGET),cross)
	CC = arm-linux-gnueabihf-gcc
else
        CC = gcc
endif


all: --motord
test: --motord --libdevice_kit

--motord: motord.c
	$(CC) motord.c $(LDFLAGS) -o motord
--libdevice_kit:
	$(CC) -shared ./mocks/libdevice_kit.c -o libdevice_kit.so

clean:
	$(shell rm rf motord libdevice_kit.so event status 2>/dev/null)
