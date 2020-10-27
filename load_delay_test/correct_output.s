	.text
	.abicalls
	.option	pic0
	.section	.mdebug.abi32,"",@progbits
	.nan	legacy
	.module	softfloat
	.text
	.file	"load_delay_test.c0pgnlkt-cgu.0"
	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	2
	.type	main,@function
	.set	nomicromips
	.set	nomips16
	.ent	main
main:
	.frame	$fp,8,$ra
	.mask 	0xc0000000,-4
	.fmask	0x00000000,0
	.set	noreorder
	.set	nomacro
	.set	noat
	addiu	$sp, $sp, -8
	sw	$ra, 4($sp)
	sw	$fp, 0($sp)
	move	$fp, $sp
	move	$sp, $fp
	lw	$fp, 0($sp)
    ; This is followed by a `nop` to allow the `lw` to finish before jumping to `$ra`
	lw	$ra, 4($sp)
	nop
	jr	$ra
	addiu	$sp, $sp, 8
	.set	at
	.set	macro
	.set	reorder
	.end	main
$func_end0:
	.size	main, ($func_end0)-main

	.section	".note.GNU-stack","",@progbits
	.text
