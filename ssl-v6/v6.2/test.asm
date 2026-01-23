; SSL v6.2 - Test Program Generated Output
; Simple program that returns 42

EXTRN ExitProcess:PROC

.code

main PROC
    push rbp
    mov rbp, rsp
    
    ; Return value 42
    mov rax, 42
    
    pop rbp
    ret
main ENDP

END
