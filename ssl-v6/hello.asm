; SSL v6.0 Compiled Output
; Hello World Program
; Target: x64 Windows

EXTRN ExitProcess:PROC

.data

.code

main PROC
    push rbp
    mov rbp, rsp
    sub rsp, 32

    ; Return success code (0)
    mov rcx, 0
    call ExitProcess

main ENDP

END
