ENTRY(_boot)

PHDRS
{
    segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
    segment_rw PT_LOAD FLAGS(6); /* 6 == RW */
}

/* RPI 4 loader address */
_boot = 0x80000;

SECTIONS
{
    . =  _boot;
   _boot = .;
   _text_boot = .;

    .text :
    {
        KEEP(*(.text.boot))
        *(.text*)
    } :segment_rx

    .rodata : ALIGN(8) { *(.rodata*) } :segment_rx
    .got    : ALIGN(8) { *(.got)     } :segment_rx

    .data : { *(.data*) } :segment_rw

    .bss : ALIGN(16)
    {
        bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        bss_end = .;
    }
}
