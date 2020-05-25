LDFLAGS=-ldl -pthread -w 

ifeq ($(TARGET),cross)
	CC = arm-linux-gnueabihf-gcc
else
        CC = gcc
endif


all: --motord
test: --motord --libdevice_kit

--motord: motord.c
	$(CC) -o motord motord.c $(LDFLAGS)
--libdevice_kit:
	$(CC) -shared libdevice_kit.c -o libdevice_kit.so

clean:
	$(shell rm rf motord libdevice_kit.so libdevice_kit.o event status 2>/dev/null)
