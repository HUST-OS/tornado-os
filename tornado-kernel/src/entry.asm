    .section .text.entry
    .globl _start
_start:
    # 初始化启动页
    la t0, boot_page_table # 此时pc是物理地址，得到的是物理地址的偏移
    srli t0, t0, 12
    li t1, (8 << 60) # 8代表Sv39模式
    or t0, t0, t1
    csrw satp, t0 # 写入 satp 
    sfence.vma # 更新页表缓存

    # 加载栈地址
    la  sp, boot_stack_top

    # 跳转到rust_main
    j  rust_main

    .section .bss.stack
    .global boot_stack
boot_stack:
    .space 4096 * 16
    .global boot_stack_top
boot_stack_top:

    .section .data
    .align 12
boot_page_table:
    .quad 0
    .quad 0
    # 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
    .zero 508 * 8
    # 第 511 项：0xffff_ffff_c000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
