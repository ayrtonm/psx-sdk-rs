NAME = test

RUSTC = rustc

CROSS = /opt/psx-tools/bin/mipsel-unknown-elf-
LD = $(CROSS)ld
CC = $(CROSS)gcc
OBJCPY = $(CROSS)objcopy

RUSTFLAGS = -O -L lib/ --target mipsel-unknown-linux-gnu -C soft-float
RUSTFLAGS += -C lto -C target-cpu=mips32 -C relocation-model=static
RUSTFLAGS += -C no-stack-check

.SUFFIXES: .o .rs .c

all: $(NAME).psexe

$(NAME).psexe: $(NAME).elf
	$(OBJCPY) -O binary $< $@

$(NAME).elf: psx.ld $(NAME).o
	$(LD) --gc-sections -o $@ -T $^

.rs.o:
	$(RUSTC) $(RUSTFLAGS) --emit obj -o $@ $<

.PHONY: clean

clean:
	rm -f *.o $(NAME).o $(NAME).elf $(NAME).psexe
