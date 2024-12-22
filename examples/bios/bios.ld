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
    } > RAM AT> ROM

    . = LOADADDR(.ram) + SIZEOF(.ram);
    . = ALIGN(4);
    __data_end = .;

    . = ADDR(.ram) + SIZEOF(.ram);
    . = ALIGN(4);
    __bss_start = .;
    .bss (NOLOAD) : {
        *(.bss*)
    }
    . = ADDR(.ram) + SIZEOF(.ram) + SIZEOF(.bss);
    . = ALIGN(4);
    __bss_end = .;


    .rom_padding : {
        . = ALIGN(512K) - 1;
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
