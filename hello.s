# some boilerplate code to tell the assembler what to do
.global _start
.intel_syntax noprefix

_start:
        # sys_write
        mov rax, 1
        mov rdi, 1
        lea rsi, [hello_world] # lea: load effective address
        mov rdx, 14
        syscall

        # sys_exit
        mov rax, 60     # rax code 60 for system exit
        mov rdi, 69     # error code for syscall code_exit
        syscall

hello_world:
        .asciz  "Hello, World!\n"
