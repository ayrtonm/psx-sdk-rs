include ../config.mk

LIBPSX_SRC := $(shell find libpsx -name '*.rs')

all: libcore.rlib libpsx.rlib

libcore.rlib:
	$(RUSTC) $(RUSTFLAGS) -o $@ $(LIBCORE_SRC)

libpsx.rlib: $(LIBPSX_SRC)
	$(RUSTC) $(RUSTFLAGS) --extern core=libcore.rlib -o $@ libpsx/lib.rs

clean:
	rm -f libcore.rlib libpsx.rlib
