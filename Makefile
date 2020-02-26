NAME=motor

CC=arm-linux-gnueabihf-gcc
SRCS=$(wildcard *.c)
CFLAGS += -Wall -Wextra
LDFLAGS += -L . $(addprefix -l:,$(wildcard lib/*))

all: $(NAME)

$(NAME):
	$(CC) $(SRCS) $(CFLAGS) $(LDFLAGS) -o $(NAME)

.PHONY: clean
clean:
	rm -f $(NAME)

.PHONY: re
re: clean all
