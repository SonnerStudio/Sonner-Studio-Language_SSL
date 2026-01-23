# Phase 4.2 - Minimal Viable Compiler

## Ziel

Ein funktionierender Compiler, der **einfache SSL-Programme** in **native x64 Executables** übersetzen kann.

## Scope

### ✅ Unterstützte Features
- **Integers**: `42`, `0`, `-5`
- **Binary Operations**: `+`, `-`, `*`, `/`
- **Variablen**: `let x = 42`
- **Einfache Funktionen**: `fn add(a: Int, b: Int) -> Int { a + b }`
- **Return Statements**: `return expr`
- **Function Calls**: `add(5, 3)`

### ⏳ Später (Phase 4.3)
- Strings
- If/Else
- While Loops
- Arrays/Lists
- Structs/Enums

## Implementierungs-Strategie

### Ansatz: End-to-End Pipeline

```
Simple SSL Source (.ssl)
    ↓
[Simplified Lexer] → Tokens
    ↓
[Simplified Parser] → Mini-AST
    ↓
[Direct Codegen] → x64 Assembly
    ↓
[MASM/Link] → Executable (.exe)
```

**Vorteil**: Schnell zum funktionierenden Ergebnis

## Test-Programme

### Test 1: Return Constant
```ssl
fn main() -> Int {
    return 42
}
```

**Expected Assembly**:
```asm
main PROC
    mov rax, 42
    ret
main ENDP
```

### Test 2: Simple Addition
```ssl
fn main() -> Int {
    return 5 + 3
}
```

**Expected Assembly**:
```asm
main PROC
    mov rax, 5
    add rax, 3
    ret
main ENDP
```

### Test 3: Variable
```ssl
fn main() -> Int {
    let x = 10
    return x
}
```

**Expected Assembly**:
```asm
main PROC
    push rbp
    mov rbp, rsp
    sub rsp, 16
    
    mov QWORD PTR [rbp-8], 10  ; x
    mov rax, QWORD PTR [rbp-8]  ; load x
    
    mov rsp, rbp
    pop rbp
    ret
main ENDP
```

### Test 4: Simple Function
```ssl
fn add(a: Int, b: Int) -> Int {
    return a + b
}

fn main() -> Int {
    return add(5, 3)
}
```

**Expected Assembly**:
```asm
add PROC
    mov rax, rcx      ; a (1st param)
    add rax, rdx      ; b (2nd param)
    ret
add ENDP

main PROC
    mov rcx, 5
    mov rdx, 3
    call add
    ret
main ENDP
```

## Implementation Steps

1. **Simplified Tokenizer** - Nur Integers, Operators, Keywords
2. **Minimal Parser** - Nur Expressions & Return
3. **Symbol Table** - Track variables & functions
4. **Code Generator** - Direct x64 emission
5. **Integration** - Complete pipeline

## Success Criteria

- [ ] Test 1 kompiliert und läuft
- [ ] Test 2 kompiliert und läuft
- [ ] Test 3 kompiliert und läuft
- [ ] Test 4 kompiliert und läuft
- [ ] Exit codes korrekt (0-255)

---

**Status**: Ready to implement  
**Estimated Time**: 1-2 hours  
**Created**: 2025-12-07 13:22
