RAM_BASE = 0x80000000;
RAM_SIZE = 2M;
BIOS_SIZE = 64K;
HEADER_SIZE = 2K;
LOAD_ADDR = RAM_BASE + BIOS_SIZE;
STACK_INIT = RAM_BASE + 0x001FFF00;

MEMORY {
    HEADER    : ORIGIN = RAM_BASE + BIOS_SIZE - HEADER_SIZE, LENGTH = HEADER_SIZE
    RAM (rwx) : ORIGIN = RAM_BASE + BIOS_SIZE, LENGTH = RAM_SIZE - BIOS_SIZE
}

ENTRY(_start)

SECTIONS {
    .psx_exe_header : {
        /* ASCII ID "PS-X EXE" */
        KEEP(*(.psx_exe));
        /* Unused */
        LONG(0);
        LONG(0);
        /* Initial PC */
        LONG(ABSOLUTE(_start));
        /* Initial GP/R28 */
        LONG(0);
        /* Load address */
        LONG(LOAD_ADDR);
        /* File size excluding the 2K header */
        LONG(__bss_start - __text_start);
        /* Unused */
        LONG(0);
        LONG(0);
        LONG(0);
        LONG(0);
        /* Initial SP/R29 and FP/R30 base */
        LONG(STACK_INIT);
        /* Initial SP/R29 and FP/R30 offset */
        LONG(0);
        /* Unused */
        LONG(0);
        LONG(0);
        LONG(0);
        LONG(0);
        LONG(0);
        /* ASCII region marker */
        KEEP(*(.region));
        . = ALIGN(HEADER_SIZE);
    } > HEADER

    .text : {
        __text_start = .;
        *(.text*)
        __text_end = .;
    } > RAM

    /* The PSX doesn't have an MMU so we don't really care about RO vs RW */
    .data : {
        __data_start = .;
        *(.data*)
        *(.rodata*)
        *(.got)
        /* Padding to a multiple of 2K is required for loading from ISO */
        . = ALIGN(2048);
        __data_end = .;
    } > RAM

    .bss (NOLOAD) : {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    } > RAM

    /* Leak up to 3 bytes to make the heap word-aligned */
    . = ALIGN(4);
    __heap_start = .;

    /DISCARD/ : {
        *(.MIPS.abiflags)
        *(.reginfo)
    }
}
