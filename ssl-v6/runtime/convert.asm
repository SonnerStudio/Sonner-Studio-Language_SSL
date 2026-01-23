; SSL v6.1 Runtime - Type Conversions
; Convert between types (Int ↔ String, etc.)

.data
int_buffer DB 32 DUP(0)     ; Buffer for int to string conversion
digits DB "0123456789", 0

.code

; Convert integer to string
; Input: rcx = integer value
; Output: rax = pointer to string (caller must free)
ssl_int_to_string PROC
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 32
    
    mov r12, rcx         ; value
    xor r13, r13         ; negative flag
    
    ; Check if negative
    test rcx, rcx
    jns .positive
    
    mov r13, 1           ; set negative flag
    neg rcx
    mov r12, rcx
    
.positive:
    ; Count digits
    mov rax, r12
    xor r14, r14         ; digit count
    
.count_loop:
    inc r14
    xor rdx, rdx
    mov rbx, 10
    div rbx
    test rax, rax
    jnz .count_loop
    
    ; Add space for minus sign if negative
    test r13, r13
    jz .no_minus
    inc r14
    
.no_minus:
    ; Allocate string (digits + null terminator)
    lea rcx, [r14 + 1]
    call ssl_malloc
    test rax, rax
    jz .alloc_failed
    
    mov r15, rax         ; result string
    
    ; Add null terminator
    mov BYTE PTR [r15 + r14], 0
    
    ; Fill digits from right to left
    mov rax, r12
    dec r14
    
.digit_loop:
    xor rdx, rdx
    mov rbx, 10
    div rbx
    
    ; rdx contains digit (0-9)
    add dl, '0'
    mov BYTE PTR [r15 + r14], dl
    
    dec r14
    test rax, rax
    jnz .digit_loop
    
    ; Add minus sign if negative
    test r13, r13
    jz .done
    mov BYTE PTR [r15], '-'
    
    jmp .done
    
.alloc_failed:
    xor rax, rax
    
.done:
    add rsp, 32
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
ssl_int_to_string ENDP

; Convert string to integer
; Input: rcx = pointer to string
; Output: rax = integer value, rdx = 1 if success, 0 if failed
ssl_string_to_int PROC
    push rbx
    push r12
    push r13
    
    mov r12, rcx         ; string
    xor rax, rax         ; result
    xor r13, r13         ; negative flag
    xor rbx, rbx         ; index
    
    ; Check for minus sign
    cmp BYTE PTR [r12], '-'
    jne .parse_loop
    
    mov r13, 1
    inc rbx
    
.parse_loop:
    movzx rdx, BYTE PTR [r12 + rbx]
    
    ; Check if digit
    cmp dl, '0'
    jb .done_success
    cmp dl, '9'
    ja .done_failed
    
    ; Convert char to digit
    sub dl, '0'
    
    ; result = result * 10 + digit
    imul rax, 10
    add rax, rdx
    
    inc rbx
    jmp .parse_loop
    
.done_success:
    ; Apply sign
    test r13, r13
    jz .positive
    neg rax
    
.positive:
    mov rdx, 1           ; success
    jmp .done
    
.done_failed:
    xor rax, rax
    xor rdx, rdx         ; failed
    
.done:
    pop r13
    pop r12
    pop rbx
    ret
ssl_string_to_int ENDP

; Convert float to string (simplified)
; Input: xmm0 = float value
; Output: rax = pointer to string
ssl_float_to_string PROC
    ; Simplified implementation - would need proper float conversion
    ; For now, return "0.0"
    
    mov rcx, 4
    call ssl_malloc
    test rax, rax
    jz .done
    
    mov BYTE PTR [rax], '0'
    mov BYTE PTR [rax + 1], '.'
    mov BYTE PTR [rax + 2], '0'
    mov BYTE PTR [rax + 3], 0
    
.done:
    ret
ssl_float_to_string ENDP

; External dependencies
EXTRN ssl_malloc:PROC

END
