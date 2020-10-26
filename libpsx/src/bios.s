.set mips1
.set noreorder
.set noat
.set nomacro

.text

.section .text.asm_printf
.global asm_printf
.type asm_printf, function

asm_printf:
	j  0xa0
	li $t1, 0x3f

.section .text.asm_flush_cache
.global asm_flush_cache
.type asm_flush_cache, function

asm_flush_cache:
	j  0xa0
	li $t1, 0x44

.section .text.asm_gpu_sync
.global asm_gpu_sync
.type asm_gpu_sync, function

asm_gpu_sync:
	j  0xa0
	li $t1, 0x4e

.section .text.asm_gpu_command_word
.global asm_gpu_command_word
.type asm_gpu_command_word, function

asm_gpu_command_word:
	j  0xa0
	li $t1, 0x49

.section .text.asm_gpu_command_word_and_params
.global asm_gpu_command_word_and_params
.type asm_gpu_command_word_and_params, function

asm_gpu_command_word_and_params:
	j  0xa0
	li $t1, 0x4a

.section .text.asm_gpu_gp1_command_word
.global asm_gpu_gp1_command_word
.type asm_gpu_gp1_command_word, function

asm_gpu_gp1_command_word:
	j  0xa0
	li $t1, 0x48

.section .text.asm_gpu_get_status
.global asm_gpu_get_status
.type asm_gpu_get_status, function

asm_gpu_get_status:
	j  0xa0
	li $t1, 0x4d

.section .text.asm_kernel_redirect
.global asm_kernel_redirect
.type asm_kernel_redirect, function

asm_kernel_redirect:
	j  0xc0
	li $t1, 0x1b
