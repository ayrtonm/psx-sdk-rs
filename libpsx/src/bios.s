.set mips1
.set noreorder
.set noat
.set nomacro
.text

.section .text.asm_printf
.global asm_printf
.type asm_printf, function

asm_printf:
j 0xA0
li $t1, 0x3F

.section .text.asm_gpu_gp1_command_word
.global asm_gpu_gp1_command_word
.type asm_gpu_gp1_command_word, function

asm_gpu_gp1_command_word:
j 0xA0
li $t1, 0x48

.section .text.asm_gpu_command_word
.global asm_gpu_command_word
.type asm_gpu_command_word, function

asm_gpu_command_word:
j 0xA0
li $t1, 0x49

.section .text.asm_gpu_command_word_params
.global asm_gpu_command_word_params
.type asm_gpu_command_word_params, function

asm_gpu_command_word_params:
j 0xA0
li $t1, 0x4A

.section .text.asm_gpu_get_status
.global asm_gpu_get_status
.type asm_gpu_get_status, function

asm_gpu_get_status:
j 0xA0
li $t1, 0x4D

