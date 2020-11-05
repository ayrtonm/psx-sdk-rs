.set mips1
.set noreorder
.set noat
.set nomacro
.text

.section .text.asm_malloc
.global asm_malloc
.type asm_malloc, function

asm_malloc:
li $v0, 0x43
mtc0 $v0, $12
mtc2 $v0, $12
nop
