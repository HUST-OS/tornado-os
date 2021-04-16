OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;

    shared_start = .;

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

    .data : ALIGN(4K) {
        _sidata = LOADADDR(.data);
        _sdata = .;
        PROVIDE(__global_pointer$ = . + 0x800);
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*)
        _edata = .;
    }

    .bss (NOLOAD) : ALIGN(4K)  {
        *(.bss.stack)
        _sbss = .;
        *(.sbss .bss .bss.*)
        _ebss = .;
    }

    /* 将一些共享的函数放到这个段方便内核或用户访问 */

    .shared_data : ALIGN(4K) {
        _sshared_data = .;
        *(.shared_data .shared_data.*)
        _eshared_data = .;
    }

    .shared_text : ALIGN(4K) {
        _sshared_text = .;
        *(.shared_text .shared_text.*)
        _eshared_text = .;
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}
