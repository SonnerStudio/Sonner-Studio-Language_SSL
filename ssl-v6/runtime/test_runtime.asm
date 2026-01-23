; SSL v6.1 Runtime - Test Suite for Memory & String Functions
; Windows x64 Assembly Test

EXTRN ExitProcess:PROC
EXTRN ssl_init_heap:PROC
EXTRN ssl_malloc:PROC
EXTRN ssl_free:PROC
EXTRN ssl_string_length:PROC
EXTRN ssl_string_concat:PROC
EXTRN ssl_string_char_at:PROC
EXTRN ssl_string_substring:PROC

.data
test_str1 DB "Hello", 0
test_str2 DB " World", 0
test_result_msg DB "Tests completed", 0

.code

main PROC
    sub rsp, 40
    
    ; Initialize heap
    call ssl_init_heap
    
    ; Test 1: malloc/free
    mov rcx, 100
    call ssl_malloc
    test rax, rax
    jz test_failed
    mov rcx, rax
    call ssl_free
    
    ; Test 2: string_length
    lea rcx, [test_str1]
    call ssl_string_length
    cmp rax, 5
    jne test_failed
    
    ; Test 3: string_concat
    lea rcx, [test_str1]
    lea rdx, [test_str2]
    call ssl_string_concat
    test rax, rax
    jz test_failed
    
    ; String should be "Hello World" (11 chars)
    mov rcx, rax
    push rax
    call ssl_string_length
    pop rcx
    
    push rax
    cmp rax, 11
    pop rax
    jne test_failed
    
    ; Free concatenated string
    call ssl_free
    
    ; Test 4: char_at
    lea rcx, [test_str1]
    mov rdx, 0
    call ssl_string_char_at
    cmp al, 'H'
    jne test_failed
    
    ; Test 5: substring
    lea rcx, [test_str1]
    mov rdx, 1          ; start = 1
    mov r8, 4           ; end = 4
    call ssl_string_substring
    test rax, rax
    jz test_failed
    
    ; Should be "ell" (3 chars)
    mov rcx, rax
    push rax
    call ssl_string_length
    pop rcx
    
    push rax
    cmp rax, 3
    pop rax
    jne test_failed
    
    ; Free substring
    call ssl_free
    
    ; All tests passed - exit with code 0
    xor rcx, rcx
    call ExitProcess
    
test_failed:
    ; Exit with error code 1
    mov rcx, 1
    call ExitProcess
    
main ENDP

END
