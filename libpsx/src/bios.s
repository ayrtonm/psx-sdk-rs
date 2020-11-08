.set mips1
.set noreorder
.set noat
.set nomacro
.text

.section .text.asm_malloc
.global asm_malloc
.type asm_malloc, function

asm_malloc:
j 0xA0
li $t1, 0x33

.section .text.asm_free
.global asm_free
.type asm_free, function

asm_free:
j 0xA0
li $t1, 0x34

.section .text.asm_calloc
.global asm_calloc
.type asm_calloc, function

asm_calloc:
j 0xA0
li $t1, 0x37

.section .text.asm_realloc
.global asm_realloc
.type asm_realloc, function

asm_realloc:
j 0xA0
li $t1, 0x38

.section .text.asm_init_heap
.global asm_init_heap
.type asm_init_heap, function

asm_init_heap:
j 0xA0
li $t1, 0x39

.section .text.asm_printf
.global asm_printf
.type asm_printf, function

asm_printf:
j 0xA0
li $t1, 0x3F

.section .text.asm_gpu_send_dma
.global asm_gpu_send_dma
.type asm_gpu_send_dma, function

asm_gpu_send_dma:
j 0xA0
li $t1, 0x47

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

