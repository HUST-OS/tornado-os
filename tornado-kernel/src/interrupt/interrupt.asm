# todo: in 64 bit, .dword; in 32 bit, .word
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

	.pushsection .text 
interrupt_reserved:
1:	j 1b
