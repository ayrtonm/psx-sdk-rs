.set mips1
.set noreorder
.set noat
.set nomacro

.text

.section .text.bios_printf
.global bios_printf
.type bios_printf, function

bios_printf:
	j  0xa0
	li $t1, 0x3f

.section .text.bios_flush_cache
.global bios_flush_cache
.type bios_flush_cache, function

bios_flush_cache:
	j  0xa0
	li $t1, 0x44

.section .text.bios_gpu_sync
.global bios_gpu_sync
.type bios_gpu_sync, function

bios_gpu_sync:
	j  0xa0
	li $t1, 0x4e

.section .text.bios_gpu_command_word
.global bios_gpu_command_word
.type bios_gpu_command_word, function

bios_gpu_command_word:
	j  0xa0
	li $t1, 0x49

.section .text.bios_gpu_command_word_and_params
.global bios_gpu_command_word_and_params
.type bios_gpu_command_word_and_params, function

bios_gpu_command_word_and_params:
	j  0xa0
	li $t1, 0x4a

.section .text.bios_gpu_gp1_command_word
.global bios_gpu_gp1_command_word
.type bios_gpu_gp1_command_word, function

bios_gpu_gp1_command_word:
	j  0xa0
	li $t1, 0x48

.section .text.bios_gpu_get_status
.global bios_gpu_get_status
.type bios_gpu_get_status, function

bios_gpu_get_status:
	j  0xa0
	li $t1, 0x4d

.section .text.bios_kernel_redirect
.global bios_kernel_redirect
.type bios_kernel_redirect, function

bios_kernel_redirect:
	j  0xc0
	li $t1, 0x1b

.section .text.asm_load_delay_test
.global asm_load_delay_test
.type asm_load_delay_test, function

#The demo app in this repo doesn't handle load delay slots, but it just so
#happens that this doesn't cause problems because $ra is only modified by
#function pro/epilogues. This function modifies ra then returns using v0
asm_load_delay_test:
    move $v0, $ra
    #when the next line is uncommented, the demo will only work if rustc handles load delay slots
    lui $ra, 0xffff
    jr $v0
    nop

