	.file	"add_one.c"
	.option nopic
	.attribute arch, "rv64i2p0_m2p0_a2p0_f2p0_d2p0_c2p0"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
	.align	1
	.globl	add_one
	.type	add_one, @function
add_one:
	addi	sp,sp,-32
	sd	s0,24(sp)
	addi	s0,sp,32
	sw	a0,-24(s0)
	lw	a5,-24(s0)
	addiw	a5,a5,1
	sext.w	a5,a5
	sw	a5,-24(s0)
	nop
	ld	s0,24(sp)
	addi	sp,sp,32
	jr	ra
	.size	add_one, .-add_one
	.ident	"GCC: (GNU) 9.2.0"