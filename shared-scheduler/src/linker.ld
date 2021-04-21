OUTPUT_ARCH(riscv)
ENTRY(_raw_table) /* Hack: keep raw table at entry address */
BASE_ADDRESS = 0x86000000;

SECTIONS
{
    . = BASE_ADDRESS;

    .data : ALIGN(4K) {
        _sidata = LOADADDR(.data);
        _sdata = .;
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*)
        _edata = .;
    }

    .text : ALIGN(4K) {
        _stext = .;
        *(.text.entry)
        *(.text .text.*)
        _etext = .;
    }
    . = ALIGN(4K);
    
    .rodata : ALIGN(4K) {
        _srodata = .;
        *(.rodata .rodata.*)
        _erodata = .;
    }

    .bss (NOLOAD) : ALIGN(4K)  {
        *(.bss.stack)
        _sbss = .;
        *(.sbss .bss .bss.*)
        _ebss = .;
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}
