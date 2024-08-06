MEMORY
{
    RAM : ORIGIN = 0x80200000, LENGTH = 128K
}

SECTIONS
{
    .text : {
        *(.text.entry)
        *(.text .text.*)
    } > RAM

    .rodata : {
        *(.rodata .rodata.*)
    } > RAM

    .data : {
        *(.data .data.*)
    } > RAM

    .bss : {
        *(.bss .bss.*)
    } > RAM

    .stack (NOLOAD) : {
        . = ALIGN(8);
        _stack_start = .;
        . = . + 8K;
        _stack_end = .;
    } > RAM
}