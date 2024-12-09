/*
    Multi
line
    comment
    test
*/


    mov r1, r0
    mov r2, r0
    mov r3, r0
    mov r4, r0
    mov r5, r0
    mov r6, r0
    mov r7, r0
    mov r8, r0
    mov r9, r0
    mov r10, r0
    mov r11, r0
    mov r12, r0
    mov r13, r0
    mov r14, r0
    mov r15, r0
    jreli 0x12

.org 0x0020 //Inline comment
    // Regular comment
    ldi r1, 0b11010011
    ldi r2, 0x55
    ldi r3, 0xAA
    ldi r4, 0x81
    ldi r5, 0x42
    ldi r6, 0xFF
    ldi r1, 6
    jrelr r1
    nop //Inline comment
    nop
    ldi r1, 0x40
    ldi r2, 0xF0
    jabsr r1, r2

.org 0xF040:
    ldi r1, 0x01
    ldi r2, 0x02
    ldi r3, 0x04
    ldi r4, 0x08
    ldi r5, 0x10
    ldi r6, 0x20
    ldi r7, 0x40
    ldi r8, 0x80
    jreli -32
