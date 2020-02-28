SRCS = $(wildcard *.c)
NAME = $(addprefix lib,$(SRCS:.c=.so))

CC = gcc
CFLAGS += -fPIC -Wall -Wextra
LDFLAGS = -shared

all: $(NAME)

$(NAME):
	$(CC) $(LDFLAGS) $(SRCS) $(CFLAGS) -o $(NAME)

.PHONY: clean
clean:
	@rm -vf $(NAME)

.PHONY: re
re: clean all
