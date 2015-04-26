ELF2PSEXE = $(SDK_ROOT)/elf2psexe/target/release/elf2psexe
LDSCRIPT = $(SDK_ROOT)/psx.ld

RUSTFLAGS += -L $(SDK_ROOT)/runtime

$(NAME).psexe: $(NAME).elf
	$(ELF2PSEXE) $(REGION) $< $@

$(NAME).elf: $(LDSCRIPT) $(NAME).o
	$(LD) --gc-sections -o $@ -T $^

.rs.o:
	$(RUSTC) $(RUSTFLAGS) --emit obj -o $@ $<

clean:
	rm -f *.o $(NAME).elf $(NAME).psexe

.SUFFIXES: .o .rs
