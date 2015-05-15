ELF2PSEXE := $(SDK_ROOT)/elf2psexe/target/release/elf2psexe
LDSCRIPT := $(SDK_ROOT)/psx.ld

RUSTFLAGS += -L $(SDK_ROOT)/runtime

RUSTLIBS := $(SDK_ROOT)/runtime/libpsx.rlib $(SDK_ROOT)/runtime/libcore.rlib

APP_SRC := $(shell find . -name '*.rs')

$(NAME).psexe: $(NAME).elf
	$(ELF2PSEXE) $(REGION) $< $@

$(NAME).elf: $(LDSCRIPT) $(NAME).o
	$(LD) --gc-sections -o $@ -T $(LDSCRIPT)  $(NAME).o $(RUSTLIBS)

$(NAME).o: $(APP_SRC)
	$(RUSTC) $(RUSTFLAGS) --emit obj -o $@ main.rs

clean:
	rm -f $(NAME).o $(NAME).elf $(NAME).psexe
