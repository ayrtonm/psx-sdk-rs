	.text
	.abicalls
	.option	pic0
	.section	.mdebug.abi32,"",@progbits
	.nan	legacy
	.text
	.file	"load_delay_test.7umq98xl-cgu.0"
	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	2
	.type	main,@function
	.set	nomicromips
	.set	nomips16
	.ent	main
main:
	.cfi_startproc
	.frame	$fp,8,$ra
	.mask 	0xc0000000,-4
	.fmask	0x00000000,0
	.set	noreorder
	.set	nomacro
	.set	noat
	addiu	$sp, $sp, -8
	.cfi_def_cfa_offset 8
	sw	$ra, 4($sp)
	sw	$fp, 0($sp)
	.cfi_offset 31, -4
	.cfi_offset 30, -8
	move	$fp, $sp
	.cfi_def_cfa_register 30
	move	$sp, $fp
	lw	$fp, 0($sp)
    ; The load delay causes this to finish after `jr $ra` meaning that we jump to the wrong address
	lw	$ra, 4($sp)
	jr	$ra
	addiu	$sp, $sp, 8
	.set	at
	.set	macro
	.set	reorder
	.end	main
$func_end0:
	.size	main, ($func_end0)-main
	.cfi_endproc

	.section	".note.GNU-stack","",@progbits
	.text
