.altmacro

.macro SAVE reg, offset
    sd  \reg, \offset*8(sp)
.endm
.macro SAVE_N n
    SAVE  x\n, \n
.endm
.macro LOAD reg, offset
    ld  \reg, \offset*8(sp)
.endm
.macro LOAD_N n
    LOAD  x\n, \n
.endm

# 保存上下文
# x0~x31
# sstatus
# sepc
.macro SAVE_CONTEXT
	addi	sp, sp, -8*34
	SAVE	x1, 1
	addi	x1, sp, 8*34
	SAVE	x1, 2
    .set    n, 3
    .rept   29
        SAVE_N  %n
        .set    n, n + 1
    .endr
	csrr 	t0, sstatus
	csrr 	t1, sepc
	SAVE	t0, 32
	SAVE	t1, 33
.endm

# 恢复上下文
.macro RESTORE_CONTEXT
	LOAD	t1, 33
	csrw	sepc, t1
	LOAD	t0, 32
	csrw 	sstatus, t0
	# addi	t0, sp, 8*34
	# csrw 	sscratch, t0

	LOAD	x1, 1
    .set    n, 3
    .rept   29
        LOAD_N  %n
        .set    n, n + 1
    .endr
	LOAD	sp, 2
.endm

	.pushsection .text 
interrupt_reserved:
1:	j 1b
	.popsection

	.pushsection .text 
supervisor_timer:
	csrrw	sp, sscratch, sp
	csrr	sp, sscratch

	SAVE_CONTEXT

	# rust_supervisor_timer 接受一个 &mut TrapFrame, 返回一个 *mut TrapFrame
	mv		a0, sp
	jal 	rust_supervisor_timer
	mv		sp, a0	# a0 是返回的 *mut TrapFrame
	
	LOAD	t0, 32
	LOAD	t1, 33
	csrw 	sstatus, t0
	csrw	sepc, t1

	LOAD	x1, 1
    .set    n, 3
    .rept   29
        LOAD_N  %n
        .set    n, n + 1
    .endr
	LOAD	sp, 2

	sret
	.popsection

	.pushsection .text
supervisor_software:
	# csrr	sp, sscratch
	jal 	rust_supervisor_software
	# mv		sp, a0
	sret
	.popsection

	.pushsection .text
supervisor_external:
	# csrr	sp, sscratch
	jal 	rust_supervisor_external
	# mv		sp, a0
	sret
	.popsection

	.pushsection .text 
trap_exception:
	SAVE_CONTEXT
	mv 		a0, sp
	jal 	rust_trap_exception
	mv		sp, a0
	RESTORE_CONTEXT
	sret
	.popsection

    .pushsection .text # 不能是rodata段，页表里没法执行
    .global trap_vector_table
	.p2align 2
trap_vector_table:
	.option push 
	.option norvc
	j	trap_exception
	j	supervisor_software
	j	interrupt_reserved
	j	interrupt_reserved
	j	interrupt_reserved
	j	supervisor_timer
	j	interrupt_reserved
	j	interrupt_reserved
	j	interrupt_reserved
	j	supervisor_external
	j	interrupt_reserved
	j	interrupt_reserved
	j	interrupt_reserved
	j	interrupt_reserved
	j	interrupt_reserved
	j	interrupt_reserved
	.option pop
	.popsection
