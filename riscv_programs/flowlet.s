	.file	"flowlet.c"
	.option nopic
	.attribute arch, "rv64i2p0_m2p0_a2p0_f2p0_d2p0_c2p0"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
	.globl	state
	.section	.sbss,"aw",@nobits
	.align	3
	.type	state, @object
	.size	state, 8
state:
	.zero	8
	.section	.rodata
	.align	3
.LC0:
	.string	"w+"
	.align	3
.LC1:
	.string	"results.txt"
	.align	3
.LC2:
	.string	"%d, %d, %d\n"
	.align	3
.LC3:
	.string	"%d\n"
	.align	3
.LC4:
	.string	"%d,"
	.text
	.align	1
	.globl	write_results
	.type	write_results, @function
write_results:
	addi	sp,sp,-48
	sd	ra,40(sp)
	sd	s0,32(sp)
	addi	s0,sp,48
	sd	a0,-48(s0)
	sd	a1,-40(s0)
	lui	a5,%hi(.LC0)
	addi	a1,a5,%lo(.LC0)
	lui	a5,%hi(.LC1)
	addi	a0,a5,%lo(.LC1)
	call	fopen
	sd	a0,-32(s0)
	lw	a5,-48(s0)
	lw	a3,-44(s0)
	lw	a4,-40(s0)
	mv	a2,a5
	lui	a5,%hi(.LC2)
	addi	a1,a5,%lo(.LC2)
	ld	a0,-32(s0)
	call	fprintf
	sw	zero,-20(s0)
	j	.L2
.L5:
	lw	a5,-20(s0)
	sext.w	a4,a5
	li	a5,1
	bne	a4,a5,.L3
	lui	a5,%hi(state)
	lw	a4,-20(s0)
	slli	a4,a4,2
	addi	a5,a5,%lo(state)
	add	a5,a4,a5
	lw	a5,0(a5)
	mv	a2,a5
	lui	a5,%hi(.LC3)
	addi	a1,a5,%lo(.LC3)
	ld	a0,-32(s0)
	call	fprintf
	j	.L4
.L3:
	lui	a5,%hi(state)
	lw	a4,-20(s0)
	slli	a4,a4,2
	addi	a5,a5,%lo(state)
	add	a5,a4,a5
	lw	a5,0(a5)
	mv	a2,a5
	lui	a5,%hi(.LC4)
	addi	a1,a5,%lo(.LC4)
	ld	a0,-32(s0)
	call	fprintf
.L4:
	lw	a5,-20(s0)
	addiw	a5,a5,1
	sw	a5,-20(s0)
.L2:
	lw	a5,-20(s0)
	sext.w	a4,a5
	li	a5,1
	ble	a4,a5,.L5
	ld	a0,-32(s0)
	call	fclose
	nop
	ld	ra,40(sp)
	ld	s0,32(sp)
	addi	sp,sp,48
	jr	ra
	.size	write_results, .-write_results
	.align	1
	.globl	flowlet
	.type	flowlet, @function
flowlet:
	addi	sp,sp,-32
	sd	ra,24(sp)
	sd	s0,16(sp)
	addi	s0,sp,32
	sd	a0,-32(s0)
	sd	a1,-24(s0)
	lw	a4,-28(s0)
	lui	a5,%hi(state)
	addi	a5,a5,%lo(state)
	lw	a5,4(a5)
	subw	a5,a4,a5
	sext.w	a5,a5
	mv	a4,a5
	li	a5,5
	ble	a4,a5,.L7
	lw	a4,-32(s0)
	lui	a5,%hi(state)
	sw	a4,%lo(state)(a5)
.L7:
	lw	a4,-28(s0)
	lui	a5,%hi(state)
	addi	a5,a5,%lo(state)
	sw	a4,4(a5)
	lui	a5,%hi(state)
	lw	a5,%lo(state)(a5)
	sw	a5,-24(s0)
	ld	a0,-32(s0)
	ld	a1,-24(s0)
	call	write_results
	nop
	ld	ra,24(sp)
	ld	s0,16(sp)
	addi	sp,sp,32
	jr	ra
	.size	flowlet, .-flowlet
	.ident	"GCC: (GNU) 9.2.0"
