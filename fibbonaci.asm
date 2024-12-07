; Test Assembly Program (Fibonacci)

; Set all registers to zero (for testing)
ADC r1, r0, r0;             0
ADC r2, r0, r0;             1
ADC r3, r0, r0;             2
ADC r4, r0, r0;             3
ADC r5, r0, r0;             4
ADC r6, r0, r0;             5
ADC r7, r0, r0;             6
ADC r8, r0, r0;             7
ADC r9, r0, r0;             8
ADC r10, r0, r0;            9
ADC r11, r0, r0;            10
ADC r12, r0, r0;            11
ADC r13, r0, r0;            12
ADC r14, r0, r0;            13
ADC r15, r0, r0;            14

; Initial Values
ADI r1, 0;                  15
ADI r2, 1;                  16
ADC r3, r1, r2;             17

; If C == 1 goto Halt.
; Note, it executes the instruction after the offset given
BRC CS, 4;                  18

; Move Registers
ADC r1, r2, r0;             19
ADC r2, r3, r0;             20

; Jump to the addition (-5)
; Note, it executes the instruction after the offset given
JMP 0xFFB;                  21

; End Program
HLT;                        22
