/* RISC-V架构，4K页系统，共享调度器链接脚本 */

OUTPUT_ARCH(riscv)
ENTRY(SHARED_RAW_TABLE) /* Hack: keep raw table at entry address */
BASE_ADDRESS = 0x80600000;

SECTIONS
{
    . = BASE_ADDRESS;
    payload_compiled_start = .;

    .rodata : ALIGN(4K) {
        srodata_page = .;
        *(.meta) /* 元数据放在最开头 */
        *(.rodata .rodata.*)
        erodata_page = .;
    }

    . = ALIGN(4K);
    .data : {
        sdata_page = .;
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*)
    }

    .bss (NOLOAD) : ALIGN(4) {
        sbss = .;
        *(.sbss .bss .bss.*)
        ebss = ALIGN(4);
    }
    edata_page = .;

    .text : ALIGN(4K) {
        stext_page = .;
        *(.text.entry)
        *(.text .text.*)
        etext_page = .;
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}
