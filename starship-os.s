	.file	"main.c"
	.option nopic
	.attribute arch, "rv64i2p0_m2p0_a2p0_f2p0_d2p0_c2p0"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
	.align	1
	.globl	print
	.type	print, @function
print:
	addi	sp,sp,-64
	sd	s0,56(sp)
	addi	s0,sp,64
	sd	a0,-40(s0)
	mv	a5,a1
	sd	a2,-56(s0)
	sw	a5,-44(s0)
	li	a5,1
	sw	a5,-20(s0)
	lw	a5,-20(s0)
	ld	a4,-40(s0)
	lw	a3,-44(s0)
	ld	a2,-56(s0)
 #APP
# 9 "src/starship-os/system.c" 1
	addi sp, sp, - 32
sd a5, 0(sp)
sd a4, 8(sp)
sd a3, 16(sp)
sd a2, 24(sp)
ecall
addi sp, sp, 32

# 0 "" 2
 #NO_APP
	li	a5,1
	mv	a0,a5
	ld	s0,56(sp)
	addi	sp,sp,64
	jr	ra
	.size	print, .-print
	.section	.rodata
	.align	3
.LC0:
	.string	"fib: "
	.section	.text.main,"ax",@progbits
	.align	1
	.globl	main
	.type	main, @function
main:
	addi	sp,sp,-32
	sd	ra,24(sp)
	sd	s0,16(sp)
	addi	s0,sp,32
	li	a0,20
	call	fib
	sd	a0,-24(s0)
	ld	a5,-24(s0)
	sd	a5,-32(s0)
	addi	a5,s0,-32
	mv	a2,a5
	li	a1,1
	lui	a5,%hi(.LC0)
	addi	a0,a5,%lo(.LC0)
	call	print
 #APP
# 17 "src/starship-os/main.c" 1
	ebreak
# 0 "" 2
 #NO_APP
	li	a5,1
	mv	a0,a5
	ld	ra,24(sp)
	ld	s0,16(sp)
	addi	sp,sp,32
	jr	ra
	.size	main, .-main
	.text
	.align	1
	.globl	fib
	.type	fib, @function
fib:
	addi	sp,sp,-48
	sd	ra,40(sp)
	sd	s0,32(sp)
	sd	s1,24(sp)
	addi	s0,sp,48
	sd	a0,-40(s0)
	ld	a5,-40(s0)
	beq	a5,zero,.L6
	ld	a4,-40(s0)
	li	a5,1
	bne	a4,a5,.L7
.L6:
	ld	a5,-40(s0)
	j	.L8
.L7:
	ld	a5,-40(s0)
	addi	a5,a5,-1
	mv	a0,a5
	call	fib
	mv	s1,a0
	ld	a5,-40(s0)
	addi	a5,a5,-2
	mv	a0,a5
	call	fib
	mv	a5,a0
	add	a5,s1,a5
.L8:
	mv	a0,a5
	ld	ra,40(sp)
	ld	s0,32(sp)
	ld	s1,24(sp)
	addi	sp,sp,48
	jr	ra
	.size	fib, .-fib
	.ident	"GCC: (SiFive GCC-Metal 10.2.0-2020.12.8) 10.2.0"
