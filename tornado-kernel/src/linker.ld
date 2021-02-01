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
        _etext = .;
    }

    .rodata : ALIGN(4K) {
        _srodata = .;
        *(.rodata .rodata.*)
        _erodata = .;
    }

    .data : ALIGN(4K) {
        _sdata = .;
        *(.data .data.*)
        _edata = .;
    }

    .bss (NOLOAD) : ALIGN(4K)  {
        _sbss = .;
        *(.sbss .bss .bss.*)
        _ebss = .;
    }

    .shared_rw : ALIGN(4K) {
        _sshared_rw = .;
        *(.shared_rw .shared_rw.*)
        _eshared_rw = .;
    }

    .shared_x : ALIGN(4K) {
        _sshared_x = .;
        *(.shared_x .shared_x.*)
        _eshared_x = .;
    }
    
    . = ALIGN(4K);
    free_memory_start = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
