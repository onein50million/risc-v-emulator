//__asm__("li a1, 0x40404040404040\n"
//        "addi a2, sp, -0xFF\n"
//        "sd a1, (a2)\n"
//        "sd a2, (sp)\n"
//        "ecall\n"
//        "ebreak");
#include "system.c"


long fib(long n);

__attribute__((section(".text.main"))) int main()  {
    long fib_return = fib(20);

    print("fib: ",1, (long[]){fib_return});

    __asm__("ebreak");
    return 1;
}

// fib.c contains the following C code and fib.bin is the build result of it:
long fib(long n) {
//    print("n: ",1, (long[]){n});
    if (n == 0 || n == 1)
        return n;
    else
        return (fib(n-1) + fib(n-2));
}




