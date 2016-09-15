include ../config.mk

LIBPSX_SRC := $(shell find libpsx -name '*.rs')

all: libcore.rlib libpsx.rlib libbios.a

libcore.rlib:
	$(RUSTC) $(RUSTFLAGS) -o $@ $(LIBCORE_SRC)

libpsx.rlib: $(LIBPSX_SRC)
	$(RUSTC) $(RUSTFLAGS) --extern core=libcore.rlib -o $@ libpsx/lib.rs

libbios.a: libbios.o
	$(AR) rcs $@ $<

libbios.o: libbios.s
	$(AS) -O2 -o $@ $<

clean:
	rm -f libcore.rlib libpsx.rlib
