ENTRY(start)

SECTIONS
{
    . = 0x2080000;
    __binary_start = .;

    .text :
    {
        KEEP(*(.text.boot))
        *(.text*)
    }

    .rodata : { *(.rodata*) }

    .data : { *(.data*) }

    .bss : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*)
        . = ALIGN(16);
        __bss_end = .;

        . += 4 * 0x1000;
        __boot_stack = .;
    }

    . = ALIGN(8);
    __binary_end = .;

    /DISCARD/ : { *(.comment*) }
}
