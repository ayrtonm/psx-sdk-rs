MEMORY
{
    /* RAM through the KSEG0 region: 2MB */
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 0x200000
}

ENTRY(main);

SECTIONS
{
        /* The first 64KB are reserved for the BIOS */
        .bios :
	{
		. = 0x10000;
	} > RAM

	.text :
	{
		*(.text*)
	} > RAM

	/* The PSX doesn't have an MMU so we don't really care about
	 * RO vs RW
	 */
	.data ALIGN(4) :
	{
		*(.data*)
		*(.rodata*)
	} > RAM

	/* This section must be stored in the "memfill" part of the
	 * header to be initialized to 0 at bootup by the BIOS/loader
	 */
	.bss ALIGN(4) :
	{
		*(.bss*)
		*(COMMON)
	} > RAM
}
