; SSL v7.0 - Production x64 Assembly (Freestanding/Bare-Metal)
; Target: ZetaTron-OS-64 Kernel

.code

main PROC
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov rax, 0       ; Return value
    add rsp, 32
    pop rbp
    ret
main ENDP

END
