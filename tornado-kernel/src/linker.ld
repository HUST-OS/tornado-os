OUTPUT_ARCH(riscv)

ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;

    kernel_start = .;

    .text : {
        _stext = .;
        *(.text.entry)
        *(.text .text.*)
    }

    .rodata : {
        _srodata = .;
        *(.rodata .rodata.*)
    }

    .data : {
        _sdata = .;
        *(.data .data.*)
    }

    .bss : {
        _sbss = .;
        *(.sbss .bss .bss.*)
    }

    . = ALIGN(4K);
    kernel_end = .;
}
