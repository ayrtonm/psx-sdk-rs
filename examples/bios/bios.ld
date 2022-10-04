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

    .ram 0x80000000 : {
        *(.data*)
        *(.bss*)
    } > RAM AT> ROM


    .rom_padding : {
        . = ALIGN(512K) - 1;
        BYTE(0x00);
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
