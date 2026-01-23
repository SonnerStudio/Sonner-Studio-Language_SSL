; SSL v6.0 - Test 1 Output
; return 42

EXTRN ExitProcess:PROC

.data

.code

main PROC
    mov rax, 42
    
    ; Exit with return value
    mov rcx, rax
    call ExitProcess
main ENDP

END
