OUTPUT_ARCH(riscv)

ENTRY(_start)

/* 省点虚拟空间……假设内核最多只有1G大小 */
BASE_ADDRESS = 0xffffffffc0200000;

SECTIONS
{
    . = BASE_ADDRESS;

    kernel_start = .;

    .text : ALIGN(4K) {
        _stext = .;
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(4K);
        _swap_frame = .;
        *(.swap)
        ASSERT(. - _swap_frame <= 4K, "swap frame larger than one page");
        _etext = .;
    }

    .rodata : ALIGN(4K) {
        _srodata = .;
        *(.rodata .rodata.*)
        _erodata = .;
    }

    .data : ALIGN(4K) {
        _sidata = LOADADDR(.data);
        _sdata = .;
        PROVIDE(__global_pointer$ = . + 0x800);
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*)
        _edata = .;
    }

    .bss (NOLOAD) : ALIGN(4K)  {
        _sbss = .;
        *(.sbss .bss .bss.*)
        _ebss = .;
    }

    . = ALIGN(4K);
    free_memory_start = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
