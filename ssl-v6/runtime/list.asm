; SSL v6.2 Runtime - List Operations (Fixed)
; Windows x64 Dynamic Lists

; List structure: [capacity][length][elem_size][data...]

.code

; ssl_list_create - Create new list
; Input: rcx = elem_size, rdx = capacity
; Output: rax = list pointer
PUBLIC ssl_list_create
ssl_list_create PROC
    push rbx
    push r12
    push r13
    
    mov r12, rcx
    mov r13, rdx
    
    ; Allocate: 24 bytes header + (capacity * elem_size)
    imul r13, r12
    add r13, 24
    mov rcx, r13
    call ssl_malloc
    
    test rax, rax
    jz create_done
    
    ; Initialize
    mov QWORD PTR [rax], rdx      ; capacity
    mov QWORD PTR [rax + 8], 0    ; length
    mov QWORD PTR [rax + 16], r12 ; elem_size
    
create_done:
    pop r13
    pop r12
    pop rbx
    ret
ssl_list_create ENDP

; ssl_list_append - Append item
; Input: rcx = list, rdx = item pointer
; Output: rax = success
PUBLIC ssl_list_append
ssl_list_append PROC
    push rbx
    push r12
    push r13
    sub rsp, 32
    
    mov r12, rcx
    mov r13, rdx
    
    ; Get properties
    mov r14, QWORD PTR [r12]      ; capacity
    mov r15, QWORD PTR [r12 + 8]  ; length
    mov rbx, QWORD PTR [r12 + 16] ; elem_size
    
    ; Check capacity (skip resize for now)
    cmp r15, r14
    jge append_failed
    
    ; Calculate position
    imul r15, rbx
    add r15, 24
    add r15, r12
    
    ; Copy item
    mov rcx, r15
    mov rdx, r13
    mov r8, rbx
    call ssl_memcpy
    
    ; Increment length
    mov rax, QWORD PTR [r12 + 8]
    inc rax
    mov QWORD PTR [r12 + 8], rax
    
    mov rax, 1
    jmp append_done
    
append_failed:
    xor rax, rax
    
append_done:
    add rsp, 32
    pop r13
    pop r12
    pop rbx
    ret
ssl_list_append ENDP

; ssl_list_get - Get item pointer
; Input: rcx = list, rdx = index
; Output: rax = item pointer
PUBLIC ssl_list_get
ssl_list_get PROC
    push rbx
    
    ; Check bounds
    mov rax, QWORD PTR [rcx + 8]
    cmp rdx, rax
    jae get_failed
    
    ; Calculate position
    mov rbx, QWORD PTR [rcx + 16]
    imul rdx, rbx
    add rdx, 24
    add rdx, rcx
    
    mov rax, rdx
    pop rbx
    ret
    
get_failed:
    xor rax, rax
    pop rbx
    ret
ssl_list_get ENDP

; ssl_list_length - Get list length
; Input: rcx = list
; Output: rax = length
PUBLIC ssl_list_length
ssl_list_length PROC
    mov rax, QWORD PTR [rcx + 8]
    ret
ssl_list_length ENDP

EXTRN ssl_malloc:PROC
EXTRN ssl_memcpy:PROC

END
