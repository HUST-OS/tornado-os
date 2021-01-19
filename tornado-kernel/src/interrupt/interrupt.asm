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

.macro SAVE_CONTEXT
	# csrrw	sp, sscratch, sp
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
	SAVE	t0, 32
	csrr 	t1, sepc
	SAVE	t1, 33
.endm

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
	SAVE_CONTEXT
	mv		a0, sp
	jal 	rust_supervisor_timer
	mv		sp, a0
	RESTORE_CONTEXT
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

    .pushsection .rodata
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
