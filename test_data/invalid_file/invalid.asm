// Fibbonaci but its in C converted to ARM Assembly
// #include <stdio.h>
//
// int main() {
//     int a, b, c;
//     a = 0;
//     b = 1;
//     do {
//         c = a + b;
//         a = b;
//         b = c;
//         printf("%d", a);
//     } while (b > 0);
//     return 0;
// }

.LC0:
        .string "%d"
main:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 16
        mov     DWORD PTR [rbp-4], 0
        mov     DWORD PTR [rbp-8], 1
.L2:
        mov     edx, DWORD PTR [rbp-4]
        mov     eax, DWORD PTR [rbp-8]
        add     eax, edx
        mov     DWORD PTR [rbp-12], eax
        mov     eax, DWORD PTR [rbp-8]
        mov     DWORD PTR [rbp-4], eax
        mov     eax, DWORD PTR [rbp-12]
        mov     DWORD PTR [rbp-8], eax
        mov     eax, DWORD PTR [rbp-4]
        mov     esi, eax
        mov     edi, OFFSET FLAT:.LC0
        mov     eax, 0
        call    printf
        cmp     DWORD PTR [rbp-8], 0
        jg      .L2
        mov     eax, 0
        leave
        ret