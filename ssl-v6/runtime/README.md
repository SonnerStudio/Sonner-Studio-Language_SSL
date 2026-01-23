# SSL v6.1 Runtime - Build & Test Instructions

## Runtime Components

### memory.asm
- Heap memory management (Windows API wrappers)
- Functions: ssl_malloc, ssl_free, ssl_calloc, ssl_memcpy, ssl_memset

### string.asm
- String manipulation
- Functions: ssl_string_length, ssl_string_concat, ssl_string_char_at, ssl_string_substring, ssl_string_compare

### test_runtime.asm
- Comprehensive test suite
- Tests all memory and string functions

## Building (if ml64 available)

```powershell
# Assemble all components
ml64 /c /Fo runtime\memory.obj runtime\memory.asm
ml64 /c /Fo runtime\string.obj runtime\string.asm
ml64 /c /Fo runtime\test_runtime.obj runtime\test_runtime.asm

# Link test executable
link /subsystem:console /entry:main runtime\memory.obj runtime\string.obj runtime\test_runtime.obj /out:runtime\test_runtime.exe

# Run tests
.\runtime\test_runtime.exe
echo "Exit code: $LASTEXITCODE"
# Expected: 0 (success)
```

## Integration with SSL Compiler

### Usage in Generated Code

```asm
; In generated SSL assembly:
EXTRN ssl_malloc:PROC
EXTRN ssl_free:PROC
EXTRN ssl_string_length:PROC
EXTRN ssl_string_concat:PROC

.code

; Example: Allocate string buffer
mov rcx, 256
call ssl_malloc
; rax now contains pointer

; Example: Get string length
lea rcx, [my_string]
call ssl_string_length
; rax contains length
```

### Linking with SSL Programs

```powershell
# When compiling SSL programs, link with runtime:
link /subsystem:console /entry:main `
    program.obj `
    runtime\memory.obj `
    runtime\string.obj `
    /out:program.exe
```

## Status

✅ memory.asm - Complete (malloc, free, calloc, memcpy, memset)
✅ string.asm - Complete (length, concat, char_at, substring, compare)
✅ test_runtime.asm - Complete (5 tests)

⏳ Needs ml64/MASM to build and verify

## Next Steps

1. Build and test runtime (if ml64 available)
2. Create list.asm (dynamic arrays)
3. Create io.asm (file operations)
4. Integrate with v6.0 compiler codegen

---

**Created**: 2025-12-07 13:32
**Status**: Runtime foundation complete (pending verification)
