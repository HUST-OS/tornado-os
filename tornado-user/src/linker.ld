OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x0;

SECTIONS
{
    . = BASE_ADDRESS;

    user_start = .;

    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    . = ALIGN(4K);
    .rodata : {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4K);
    .data : {
        *(.data .data.*)
    }
    .bss : {
        sbss = .;
        *(.bss .bss.*)
        ebss = .;
    }
    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}
