.set mips1
.set noreorder
.set noat
.set nomacro

.text

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
