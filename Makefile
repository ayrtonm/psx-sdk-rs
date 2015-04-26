NAME = test

include config.mk

RUSTFLAGS = -O --target=target.json -C soft-float -C lto
RUSTFLAGS += --extern psx=runtime/libpsx.rlib

all:
	$(MAKE_COMMAND) -C elf2psexe/
	$(MAKE_COMMAND) -C runtime/
	$(MAKE_COMMAND) -C apps/

clean:
	$(MAKE_COMMAND) -C elf2psexe/ clean
	$(MAKE_COMMAND) -C runtime/ clean
	$(MAKE_COMMAND) -C apps/ clean


.PHONY: all clean
