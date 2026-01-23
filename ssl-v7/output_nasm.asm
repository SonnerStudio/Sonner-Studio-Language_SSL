; SSL v7.0 - Production x64 Assembly (Freestanding/Bare-Metal)
; Target: ZetaTron-OS-64 Kernel
; Format: ELF64 (NASM)

BITS 64
section .text

global main
main:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov rax, 42       ; Return value
    add rsp, 32
    pop rbp
    ret

