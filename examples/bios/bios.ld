MEMORY {
    RAM(rwx) : ORIGIN = 0x80000000, LENGTH = 64K
    ROM (rx) : ORIGIN = 0xBFC00000, LENGTH = 512K
}

ENTRY(boot)

SECTIONS {
    .text 0xBFC00000 : {
        __text_start = .;
        *(.text.boot)
        *(.text*)
        *(.rodata*)
        __text_end = .;
    } > ROM

    . = ALIGN(4);
    __data_start = .;

    /* Add an offset to the start of KSEG0 to avoid overlap an vectors */
    .ram 0x80000100 : {
        *(.data*)
        /* TODO: Make .bss NOLOAD */
        *(.bss*)
    } > RAM AT> ROM

    . = LOADADDR(.ram) + SIZEOF(.ram);
    . = ALIGN(4);
    __data_end = .;


    .rom_padding : {
        . = ALIGN(512K) - 1;
        /* du doesn't like files that end with all zeros, so let's add a non-zero byte */
        BYTE(0xFF);
    } > ROM

    /DISCARD/ : {
        *(.got)
        *(.MIPS.abiflags)
        *(.reginfo)
        *(.eh_frame_hdr)
        *(.eh_frame)
        *(.mdebug.abi32)
        *(.pdr)
        *(.comment)
    }
}
