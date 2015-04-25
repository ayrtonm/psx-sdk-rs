NAME = test

RUSTC = rustc

CROSS = /opt/psx-tools/bin/mipsel-unknown-elf-
LD = $(CROSS)ld
CC = $(CROSS)gcc

RUSTFLAGS = -O -L lib/ --target mipsel-unknown-linux-gnu -C soft-float
RUSTFLAGS += -C lto -C target-cpu=mips32 -C relocation-model=static
RUSTFLAGS += -C no-stack-check

# Region for the resulting executable: NA, E or J
REGION = E

.SUFFIXES: .o .rs .c

all: elf2psexe $(NAME).psexe

$(NAME).psexe: $(NAME).elf
	elf2psexe/target/release/elf2psexe $(REGION) $< $@

$(NAME).elf: psx.ld $(NAME).o
	$(LD) --gc-sections -o $@ -T $^

.rs.o:
	$(RUSTC) $(RUSTFLAGS) --emit obj -o $@ $<

.PHONY: clean

clean:
	rm -f *.o $(NAME).o $(NAME).elf $(NAME).psexe

.PHONY: elf2psexe

elf2psexe:
	$(MAKE_COMMAND) -C elf2psexe
