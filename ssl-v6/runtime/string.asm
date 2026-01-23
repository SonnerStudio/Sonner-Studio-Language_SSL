; SSL v6.2 Runtime - String Operations (Fixed)
; Windows x64 String Functions

.code

; ssl_string_length - Get string length
; Input: rcx = string pointer
; Output: rax = length
PUBLIC ssl_string_length
ssl_string_length PROC
    xor rax, rax
    test rcx, rcx
    jz done
    
check_char:
    cmp BYTE PTR [rcx + rax], 0
    je done
    inc rax
    jmp check_char
    
done:
    ret
ssl_string_length ENDP

; ssl_string_concat - Concatenate strings
; Input: rcx = str1, rdx = str2
; Output: rax = new string (caller must free)
PUBLIC ssl_string_concat
ssl_string_concat PROC
    push rbx
    push r12
    push r13
    push r14
    sub rsp, 32
    
    mov r12, rcx
    mov r13, rdx
    
    ; Get len1
    call ssl_string_length
    mov r14, rax
    
    ; Get len2
    mov rcx, r13
    call ssl_string_length
    mov rbx, rax
    
    ; Allocate len1 + len2 + 1
    lea rcx, [r14 + rbx + 1]
    call ssl_malloc
    test rax, rax
    jz concat_failed
    
    push rax
    
    ; Copy str1
    mov rcx, rax
    mov rdx, r12
    mov r8, r14
    call ssl_memcpy
    
    pop rax
    push rax
    
    ; Copy str2
    lea rcx, [rax + r14]
    mov rdx, r13
    mov r8, rbx
    call ssl_memcpy
    
    pop rax
    
    ; Null terminate
    add r14, rbx
    mov BYTE PTR [rax + r14], 0
    
concat_failed:
    add rsp, 32
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
ssl_string_concat ENDP

; ssl_string_char_at - Get character at index
; Input: rcx = string, rdx = index
; Output: al = character
PUBLIC ssl_string_char_at
ssl_string_char_at PROC
    xor rax, rax
    test rcx, rcx
    jz char_done
    
    movzx rax, BYTE PTR [rcx + rdx]
    
char_done:
    ret
ssl_string_char_at ENDP

EXTRN ssl_malloc:PROC
EXTRN ssl_memcpy:PROC

END
