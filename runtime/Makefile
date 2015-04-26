NAME = libpsx

include ../config.mk

all: libcore.rlib libpsx.rlib

libcore.rlib:
	$(RUSTC) $(RUSTFLAGS) -o $@ $(LIBCORE_SRC)

libpsx.rlib: libpsx.rs
	$(RUSTC) $(RUSTFLAGS) --extern core=libcore.rlib -o $@ $<

clean:
	rm -f libcore.rlib libpsx.rlib
