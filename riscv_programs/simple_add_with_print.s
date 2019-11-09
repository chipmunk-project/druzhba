	.file	"simple_add_with_print.c"
	.option nopic
	.attribute arch, "rv64i2p0_m2p0_a2p0_f2p0_d2p0_c2p0"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
	.section	.rodata
	.align	3
.LC0:
	.string	"Result: %d, %d, %d\n"
	.text
	.align	1
	.globl	simple_add_with_print
	.type	simple_add_with_print, @function
simple_add_with_print:
	addi	sp,sp,-32
	sd	ra,24(sp)
	sd	s0,16(sp)
	addi	s0,sp,32
	sd	a0,-32(s0)
	sd	a1,-24(s0)
	lw	a5,-32(s0)
	addiw	a5,a5,1
	sext.w	a5,a5
	sw	a5,-32(s0)
	lw	a5,-28(s0)
	addiw	a5,a5,2
	sext.w	a5,a5
	sw	a5,-28(s0)
	lw	a5,-24(s0)
	addiw	a5,a5,3
	sext.w	a5,a5
	sw	a5,-24(s0)
	lw	a5,-32(s0)
	lw	a4,-28(s0)
	lw	a3,-24(s0)
	mv	a2,a4
	mv	a1,a5
	lui	a5,%hi(.LC0)
	addi	a0,a5,%lo(.LC0)
	call	printf
	nop
	ld	ra,24(sp)
	ld	s0,16(sp)
	addi	sp,sp,32
	jr	ra
	.size	simple_add_with_print, .-simple_add_with_print
	.ident	"GCC: (GNU) 9.2.0"
