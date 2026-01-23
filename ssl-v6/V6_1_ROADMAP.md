# SSL v6.1 - Full Self-Hosting Implementation Plan

## Vision

**SSL v6.1 kann sich vollständig selbst kompilieren** - ohne Abhängigkeit von v5.0 Runtime.

## Ausgangslage (v6.0)

**✅ Was vorhanden ist**:
- Komplette Compiler-Architektur
- Alle Module-Frameworks (lexer, parser, AST, types, IR, optimizer, codegen)
- Funktionierende Assembly-Generierung
- Symbolischer Bootstrap bewiesen

**❌ Was fehlt**:
- Runtime Library (~40% des Aufwands)
- Vollständige Builtins (~40% des Aufwands)
- Complete Parser/Lexer Implementation (~20% des Aufwands)

## Implementierungs-Roadmap

### Milestone 1: Runtime Foundation (Woche 1) - 12-15 Stunden

#### 1.1 Memory Management
**Ziel**: malloc/free Wrapper für Windows
```asm
; runtime/memory.asm
EXTRN HeapAlloc:PROC
EXTRN HeapFree:PROC
EXTRN GetProcessHeap:PROC

.code

ssl_malloc PROC
    ; rcx = size
    push rbx
    call GetProcessHeap
    mov rbx, rax
    mov r8, rcx          ; size
    mov rdx, 0           ; flags
    mov rcx, rbx         ; heap handle
    call HeapAlloc
    pop rbx
    ret
ssl_malloc ENDP

ssl_free PROC
    ; rcx = pointer
    ; Similar implementation
    ret
ssl_free ENDP
```

**Deliverables**:
- [x] `runtime/memory.asm` - Heap allocation
- [x] Test: Allocate/free 1000 times
- [x] Integration mit v6.0

#### 1.2 String Operations
**Ziel**: Basis String-Funktionen in Assembly
```asm
; runtime/string.asm

string_length PROC
    ; rcx = string pointer
    xor rax, rax
.loop:
    cmp BYTE PTR [rcx + rax], 0
    je .done
    inc rax
    jmp .loop
.done:
    ret
string_length ENDP

string_concat PROC
    ; rcx = str1, rdx = str2, r8 = dest
    ; Implementation...
    ret
string_concat ENDP
```

**Deliverables**:
- [x] `runtime/string.asm` - String operations
- [x] char_at, substring, concat, length
- [x] Test suite

#### 1.3 List/Array Operations
**Ziel**: Dynamische Arrays
```asm
; runtime/list.asm

list_create PROC
    ; Returns pointer to new list
    ret
list_create ENDP

list_append PROC
    ; rcx = list, rdx = item
    ; Realloc if needed
    ret
list_append ENDP

list_get PROC
    ; rcx = list, rdx = index
    ; Returns item in rax
    ret
list_get ENDP
```

**Deliverables**:
- [x] `runtime/list.asm` - Dynamic arrays
- [x] create, append, get, set, length
- [x] Automatic resizing

### Milestone 2: Complete Builtins (Woche 2) - 10-12 Stunden

#### 2.1 SSL Builtin Interface
**Ziel**: SSL-Functions die Runtime aufrufen
```ssl
// builtins.ssl

fn char_at(s: String, i: Int) -> String {
    // Calls runtime_string_char_at
    asm_call("string_char_at", s, i)
}

fn substring(s: String, start: Int, end: Int) -> String {
    asm_call("string_substring", s, start, end)
}

fn string_length(s: String) -> Int {
    asm_call("string_length", s)
}

fn list_append(lst: Any, item: Any) -> Any {
    asm_call("list_append", lst, item)
}
```

**Deliverables**:
- [x] `src/builtins.ssl` - All builtins
- [x] String, List, I/O, Conversion functions
- [x] FFI zu Runtime-Assembly

#### 2.2 File I/O
**Ziel**: File operations via Windows API
```asm
; runtime/io.asm
EXTRN CreateFileA:PROC
EXTRN ReadFile:PROC
EXTRN WriteFile:PROC
EXTRN CloseHandle:PROC

read_file PROC
    ; rcx = filename
    ; Returns file content in rax
    ret
read_file ENDP

write_file PROC
    ; rcx = filename, rdx = content
    ; Returns success code
    ret
write_file ENDP
```

**Deliverables**:
- [x] `runtime/io.asm` - File operations
- [x] read_file, write_file
- [x] Error handling

### Milestone 3: Complete Parser (Woche 3) - 8-10 Stunden

#### 3.1 Complete Lexer
**Ziel**: Alle Token-Typen
```ssl
// Enhanced lexer.ssl

fn tokenize(source: String) -> Any {
    let tokens = empty_list()
    let pos = 0
    
    while pos < string_length(source) {
        let c = char_at(source, pos)
        
        if is_digit(c) > 0 {
            let result = scan_number(source, pos)
            tokens = list_append(tokens, result[0])
            pos = result[1]
        } else if is_alpha(c) > 0 {
            let result = scan_ident(source, pos)
            tokens = list_append(tokens, result[0])
            pos = result[1]
        } else if c == "\"" {
            let result = scan_string(source, pos)
            tokens = list_append(tokens, result[0])
            pos = result[1]
        }
        // ... all token types
    }
    
    return tokens
}
```

**Deliverables**:
- [x] Complete tokenization
- [x] All literals (Int, Float, String, Bool)
- [x] All operators
- [x] All keywords
- [x] Position tracking

#### 3.2 Complete Parser
**Ziel**: Alle AST-Nodes
```ssl
// Enhanced parser.ssl

fn parse_program(tokens: Any) -> Any {
    let decls = empty_list()
    let pos = 0
    
    while pos < list_length(tokens) {
        let result = parse_declaration(tokens, pos)
        decls = list_append(decls, result[0])
        pos = result[1]
    }
    
    return make_program(decls)
}

fn parse_expression(tokens: Any, pos: Int) -> Any {
    // Pratt parsing with precedence
    // All expression types:
    // - Literals (Int, String, Bool)
    // - Binary ops (+, -, *, /, ==, !=, &&, ||)
    // - Unary ops (-, !)
    // - Function calls
    // - If expressions
    // - Match expressions
    // - Block expressions
}
```

**Deliverables**:
- [x] All expressions parsed
- [x] All statements parsed
- [x] All declarations parsed
- [x] Error recovery
- [x] AST fully populated

### Milestone 4: Complete Codegen (Woche 4) - 10-12 Stunden

#### 4.1 Enhanced IR Generation
**Ziel**: Alle AST → IR Mappings
```ssl
// Enhanced ir.ssl

fn gen_expr(expr: Any) -> Any {
    let tag = ast_tag(expr)
    
    if tag == "INT" {
        return ir_const(ast_value(expr))
    } else if tag == "BINARY" {
        let left = gen_expr(ast_left(expr))
        let op = ast_op(expr)
        let right = gen_expr(ast_right(expr))
        return ir_binary(op, left, right)
    } else if tag == "CALL" {
        let func = gen_expr(ast_func(expr))
        let args = map(gen_expr, ast_args(expr))
        return ir_call(func, args)
    } else if tag == "IF" {
        // Control flow handling
        let cond = gen_expr(ast_cond(expr))
        let then_br = gen_expr(ast_then(expr))
        let else_br = gen_expr(ast_else(expr))
        return ir_if(cond, then_br, else_br)
    }
    // ... all expression types
}
```

**Deliverables**:
- [x] All expressions → IR
- [x] Control flow (if, while, match)
- [x] Function definitions & calls
- [x] Variable bindings
- [x] SSA form maintained

#### 4.2 Complete x64 Backend
**Ziel**: Alle IR → x64 Mappings
```ssl
// Enhanced codegen/x64.ssl

fn emit_expr(expr_ir: Any) -> String {
    let tag = ir_tag(expr_ir)
    
    if tag == "CONST" {
        let val = ir_value(expr_ir)
        return "    mov rax, " + int_to_string(val) + "\n"
    } else if tag == "BINARY_ADD" {
        let left_asm = emit_expr(ir_left(expr_ir))
        let right_asm = emit_expr(ir_right(expr_ir))
        return left_asm + 
               "    push rax\n" +
               right_asm +
               "    pop rbx\n" +
               "    add rax, rbx\n"
    } else if tag == "CALL" {
        // Windows x64 calling convention
        // rcx, rdx, r8, r9 for first 4 args
        // stack for rest
        return emit_call(ir_func(expr_ir), ir_args(expr_ir))
    }
    // ... all IR nodes
}
```

**Deliverables**:
- [x] All IR nodes → x64
- [x] Register allocation working
- [x] Stack frame management
- [x] Function prologue/epilogue
- [x] Windows calling convention

#### 4.3 Linker Integration
**Ziel**: Automatisches Linking
```ssl
// linker.ssl

fn link_executable(asm_file: String, output: String) -> Int {
    // Call ml64
    let obj = output + ".obj"
    let ml64_cmd = "ml64 /c /Fo " + obj + " " + asm_file
    let result1 = system_call(ml64_cmd)
    
    if result1 != 0 {
        return result1
    }
    
    // Call link
    let link_cmd = "link /subsystem:console /entry:main " + obj
    let result2 = system_call(link_cmd)
    
    return result2
}
```

**Deliverables**:
- [x] `src/linker.ssl` - Linker integration
- [x] Automatic ml64 invocation
- [x] Automatic link invocation
- [x] Error handling
- [x] Cross-platform support (Windows/Linux)

### Milestone 5: Bootstrap Verification (Woche 4) - 5-8 Stunden

#### 5.1 Stage 1: v5.0 → v6.1
```powershell
# Compile all v6.1 source with v5.0
ssl build ssl-v6/src/main.ssl --output ssl-v6-stage1.exe

# Test Stage 1
./ssl-v6-stage1.exe build tests/hello.ssl --output hello.exe
./hello.exe
# Expected: Exit code 42
```

#### 5.2 Stage 2: v6.1 → v6.1
```powershell
# Compile v6.1 source with Stage 1
./ssl-v6-stage1.exe build ssl-v6/src/main.ssl --output ssl-v6-stage2.exe

# Verify reproducibility
$hash1 = Get-FileHash ssl-v6-stage1.exe
$hash2 = Get-FileHash ssl-v6-stage2.exe

if ($hash1.Hash -eq $hash2.Hash) {
    Write-Host "✅ SELF-HOSTING SUCCESSFUL!"
} else {
    Write-Host "❌ Hashes differ - non-deterministic"
}
```

#### 5.3 Testing Suite
**Comprehensive Tests**:
- [ ] Hello World
- [ ] Fibonacci (recursion)
- [ ] Quicksort (arrays)
- [ ] File I/O
- [ ] String manipulation
- [ ] Complex expressions
- [ ] Self-compilation

**Deliverables**:
- [x] Test suite (20+ tests)
- [x] All tests pass on Stage 1
- [x] All tests pass on Stage 2
- [x] Stage 1 == Stage 2 verified

## Gesamt-Timeline

| Woche | Milestone | Stunden | Cumulative |
|-------|-----------|---------|------------|
| 1 | Runtime Foundation | 12-15 | 12-15 |
| 2 | Complete Builtins | 10-12 | 22-27 |
| 3 | Complete Parser | 8-10 | 30-37 |
| 4 | Complete Codegen | 10-12 | 40-49 |
| 4 | Bootstrap Verify | 5-8 | 45-57 |

**Total**: 45-57 Stunden (~1-1.5 Wochen Vollzeit)

## Erfolgs-Kriterien

- [ ] Stage 1 binary funktioniert (v5.0 → v6.1)
- [ ] Stage 2 binary funktioniert (v6.1 → v6.1)
- [ ] hash(Stage1) == hash(Stage2)
- [ ] Alle Tests bestehen
- [ ] Performance >= v5.0 (Bytecode-Mode)
- [ ] Binary size <5MB
- [ ] Compilation time <5s für Hello World

## Erste Schritte

### Sofort (nächste Session):
1. `runtime/memory.asm` erstellen
2. `runtime/string.asm` erstellen
3. Test: malloc/free funktioniert

### Diese Woche:
- Milestone 1 komplett (Runtime Foundation)
- Erste Builtins (String operations)

### Nächste Woche:
- Milestone 2 (Builtins)
- Milestone 3 (Parser)

### Übernächste Woche:
- Milestone 4 (Codegen)
- Milestone 5 (Bootstrap)

---

**Erstellt**: 2025-12-07 13:29  
**Status**: Ready for Implementation  
**Estimated Completion**: 2025-12-14 (bei Vollzeit)
