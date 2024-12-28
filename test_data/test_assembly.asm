/*
This is a test assembly program that calculates the Fibonacci sequence.
*/

LDI r1, 0 // Loads 0 into r1
LDI r2, 1 // Loads 1 into r2
LDI r3, 0 // Loads 0 into r3
.loop
    ADD r1, r2, r3
    BRH CS, .done
    MOV r2, r1
    MOV r3, r2
    JMP .loop
.done
    HLT