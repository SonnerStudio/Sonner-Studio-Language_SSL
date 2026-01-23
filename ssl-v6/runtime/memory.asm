; SSL v6.2 Runtime - Memory Management (Fixed)
; Windows x64 Heap Allocation

EXTRN GetProcessHeap:PROC
EXTRN HeapAlloc:PROC
EXTRN HeapFree:PROC

.data
heap_handle DQ 0

.code

; ssl_malloc - Allocate memory
; Input: rcx = size
; Output: rax = pointer (or 0)
PUBLIC ssl_malloc
ssl_malloc PROC
    push rbx
    push r12
    
    mov r12, rcx
    
    ; Get or initialize heap
    mov rax, [heap_handle]
    test rax, rax
    jnz have_heap
    
    call GetProcessHeap
    mov [heap_handle], rax
    
have_heap:
    mov r8, r12
    xor rdx, rdx
    mov rcx, rax
    call HeapAlloc
    
    pop r12
    pop rbx
    ret
ssl_malloc ENDP

; ssl_free - Free memory
; Input: rcx = pointer  
; Output: rax = success
PUBLIC ssl_free
ssl_free PROC
    push rbx
    
    test rcx, rcx
    jz is_null
    
    mov rbx, rcx
    mov rax, [heap_handle]
    test rax, rax
    jz no_heap
    
    mov r8, rbx
    xor rdx, rdx
    mov rcx, rax
    call HeapFree
    
    pop rbx
    ret
    
is_null:
no_heap:
    xor rax, rax
    pop rbx
    ret
ssl_free ENDP

; ssl_memcpy - Copy memory
; Input: rcx = dest, rdx = src, r8 = count
; Output: rax = dest
PUBLIC ssl_memcpy
ssl_memcpy PROC
    push rdi
    push rsi
    push rcx
    
    mov rdi, rcx
    mov rsi, rdx
    mov rcx, r8
    rep movsb
    
    pop rax
    pop rsi
    pop rdi
    ret
ssl_memcpy ENDP

END
