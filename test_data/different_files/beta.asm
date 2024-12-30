# Test Assembly Program (Fibonacci)
# // # comment in comment in comment :3

LDI r1, 0;  why are you not being parsed correctly????
LDI r2, 1;
LDI r3, 0;


.loooop
    ADD r1, r2, r3;
    BRH CS, .dooone;
    MOV r2, r1; // Just some comment to make the files different
    MOV r3, r2;
    JMP .loooop;


.dooone
    HLT;