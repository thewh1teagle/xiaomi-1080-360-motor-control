NAME = control
RELEASE_NAME = $(NAME)
RELEASE_CC = arm-linux-gnueabihf-gcc
RELEASE_TARGET = armv7-unknown-linux-gnueabihf
DEBUG_NAME = $(RELEASE_NAME)-debug
DEBUG_NAME = $(RELEASE_NAME)-debug
DEBUG_CC = gcc
DEBUG_TARGET = x86_64-unknown-linux-gnu


# run command: pass down arguments to binary
ifneq ( $(shell ( [ -n "$(MAKECMDGOALS)" ] || [[ "run" == "$(MAKECMDGOALS)*" ]] ) && echo y ),)
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(eval $(RUN_ARGS):;@:)
endif


all: $(DEBUG_NAME) mocks

.PHONY += setup-release
setup-release:
	rustup target add $(RELEASE_TARGET)

.PHONY += setup-debug
setup-debug:
	rustup target add $(DEBUG_TARGET)

.PHONY += mocks
mocks:
	@make -C mocks/ --no-print-directory

.PHONY += clean-mocks
clean-mocks:
	@make -C mocks/ clean --no-print-directory

$(RELEASE_NAME): setup-release
	RUSTFLAGS="-C linker=$(RELEASE_CC)" cargo build --target=$(RELEASE_TARGET) --release
	cp target/$(RELEASE_TARGET)/release/$(NAME) $(RELEASE_NAME)

$(DEBUG_NAME): setup-debug
	RUSTFLAGS="-C linker=$(DEBUG_CC)" cargo build --target=$(DEBUG_TARGET)
	cp target/$(DEBUG_TARGET)/debug/$(NAME) $(DEBUG_NAME)

.PHONY += release
release: $(RELEASE_NAME)

.PHONY += debug
debug: $(DEBUG_NAME)

.PHONY += run-release
run-release: $(RELEASE_NAME) mocks
	@echo -e "##################################################"
	LIBRARY_PATH=/mnt/data/lib ./$(RELEASE_NAME) $(RUN_ARGS)
	@echo -e "##################################################"

.PHONY += run-debug
run-debug: $(DEBUG_NAME) mocks
	@echo -e "##################################################"
	LIBRARY_PATH=./mocks ./$(DEBUG_NAME) $(RUN_ARGS)
	@echo -e "##################################################"

.PHONY += run
run: run-debug

.PHONY += clean
clean: clean-mocks
	@rm -vf $(RELEASE_NAME)
	@rm -vf $(DEBUG_NAME)
	@rm -rf target/

.PHONY += re
re: clean all

