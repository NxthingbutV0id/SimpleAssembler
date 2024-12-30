; Test Assembly Program (Fibonacci) but its all commented out

/*
    LDI r1, 0
    LDI r2, 1
    LDI r3, 0
    .loop
        ADD r1, r2, r3
        BRH CS, .done
        MOV r2, r1
        MOV r3, r2
        JMP .loop
    .done
        HLT
*/