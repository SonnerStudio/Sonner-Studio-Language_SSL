# SSL v6.2 - Full Implementation Plan

## Overview

**Goal**: Complete praktisches Self-Hosting (v6.1 → v6.1)  
**Duration**: ~40-50 hours (~1-2 Wochen Vollzeit)  
**Deliverable**: Fully functional self-hosting compiler

## Prerequisites

✅ v6.0/v6.1 Complete (Frameworks vorhanden)  
✅ Toolchain Setup (ml64, link)  
⏳ Full Implementation (This plan)

## Week-by-Week Breakdown

### Week 1: Frontend Complete Implementation (16-20h)

#### Day 1-2: Complete Lexer (6-8h)

**Goal**: Echtes Tokenizing statt Placeholder

**Tasks**:
1. **Character-by-Character Scanning** (2h)
```ssl
fn tokenize_real(source: String) -> Any {
    let tokens = list_create(8, 100)  // Token size, initial capacity
    let pos = 0
    let len = string_length(source)
    
    while pos < len {
        let c = char_at(source, pos)
        
        if is_whitespace(c) {
            pos = pos + 1
            continue
        }
        
        if is_digit(c) {
            let result = scan_number(source, pos)
            tokens = list_append(tokens, result)
            pos = get_new_pos(result)
        } else if is_alpha(c) {
            let result = scan_ident(source, pos)
            tokens = list_append(tokens, result)
            pos = get_new_pos(result)
        }
        // ... etc
    }
    
    return tokens
}
```

2. **Number Scanning** (1h)
   - Multi-digit integers
   - Floating point (optional)
   - Hex/binary literals (optional)

3. **String Scanning** (2h)
   - Escape sequences (\n, \t, \", \\)
   - Multi-line strings (optional)

4. **Operator Scanning** (1h)
   - Single char: +, -, *, /, =, <, >
   - Double char: ==, !=, <=, >=, &&, ||, ->

5. **Testing** (1h)
   - Test alle Token-Typen
   - Position tracking verifizieren

#### Day 3-4: Complete Parser (8-10h)

**Goal**: Echtes AST-Building statt Placeholder

**Tasks**:
1. **Expression Parsing with Pratt** (3h)
```ssl
fn parse_expression(tokens: Any, pos: Int, min_prec: Int) -> Any {
    // Parse prefix/primary
    let left = parse_primary(tokens, pos)
    pos = get_next_pos(left)
    
    // Parse infix operators
    while true {
        let op = peek_token(tokens, pos)
        if is_eof(op) { break }
        
        let prec = precedence(op)
        if prec < min_prec { break }
        
        pos = pos + 1  // consume operator
        let right = parse_expression(tokens, pos, prec + 1)
        left = make_binary(op, left, right)
        pos = get_next_pos(right)
    }
    
    return left
}
```

2. **Statement Parsing** (2h)
   - Let/Var declarations
   - Assignments
   - Return, Break, Continue
   - Expression statements

3. **Declaration Parsing** (2h)
   - Function definitions
   - Parameter lists
   - Return types

4. **Error Recovery** (1h)
   - Synchronization points
   - Error collection

5. **Testing** (1h)
   - Parse komplexe Programme
   - Error recovery testen

#### Day 5: IR Generator Complete (4-5h)

**Goal**: Echte SSA-Form Generierung

**Tasks**:
1. **Expression IR** (2h)
```ssl
fn gen_expr_ir(expr: Any) -> Any {
    let tag = ast_tag(expr)
    
    if tag == "INT" {
        return ir_const(ast_value(expr))
    } else if tag == "BINARY" {
        let left_ir = gen_expr_ir(ast_left(expr))
        let right_ir = gen_expr_ir(ast_right(expr))
        let op = ast_op(expr)
        return ir_binary(op, left_ir, right_ir)
    } else if tag == "CALL" {
        let func_ir = gen_expr_ir(ast_func(expr))
        let args_ir = map_gen_expr(ast_args(expr))
        return ir_call(func_ir, args_ir)
    }
    // ... all expression types
}
```

2. **Statement IR** (1h)
3. **SSA Construction** (1h)
4. **Testing** (1h)

### Week 2: Backend & Runtime (16-20h)

#### Day 1-2: Complete Codegen (8-10h)

**Goal**: Alle IR → x64 Mappings

**Tasks**:
1. **All Binary Operators** (2h)
   - Arithmetic: +, -, *, /, %
   - Comparison: ==, !=, <, >, <=, >=
   - Logical: &&, ||

2. **Control Flow** (3h)
```ssl
fn emit_if(cond_ir: Any, then_ir: Any, else_ir: Any) -> String {
    let label = next_label()
    
    let asm = ""
    asm = asm + emit_expr(cond_ir)
    asm = asm + "    test rax, rax\n"
    asm = asm + "    jz .else_" + int_to_string(label) + "\n"
    asm = asm + emit_expr(then_ir)
    asm = asm + "    jmp .endif_" + int_to_string(label) + "\n"
    asm = asm + ".else_" + int_to_string(label) + ":\n"
    asm = asm + emit_expr(else_ir)
    asm = asm + ".endif_" + int_to_string(label) + ":\n"
    
    return asm
}
```

3. **Function Calls** (2h)
   - Parameter passing (rcx, rdx, r8, r9, stack)
   - Shadow space allocation
   - Return value handling

4. **Variable Management** (1h)
   - Stack allocation
   - Register allocation (simple)

5. **Testing** (1h)

#### Day 3: Runtime Integration (4-5h)

**Tasks**:
1. **Build Runtime Library** (1h)
```powershell
cd runtime
ml64 /c *.asm
lib /OUT:ssl_runtime.lib *.obj
```

2. **Link Generated Code** (1h)
3. **Test Integration** (2h)
4. **Debug Issues** (1h)

#### Day 4-5: Linker & Build System (6-8h)

**Tasks**:
1. **Automatic Linking** (3h)
```ssl
fn link_program(obj_file: String, output: String) -> Int {
    let runtime_lib = "ssl-v6\\runtime\\ssl_runtime.lib"
    
    let cmd = "link /subsystem:console /entry:main "
    cmd = cmd + obj_file + " "
    cmd = cmd + runtime_lib + " "
    cmd = cmd + "/out:" + output
    
    return system_call(cmd)
}
```

2. **Build Command** (2h)
```ssl
fn cmd_build(args: Any) -> Int {
    let source = list_get(args, 0)
    let output = list_get(args, 1)
    
    // 1. Parse
    let tokens = tokenize(read_file(source))
    let ast = parse(tokens)
    
    // 2. Generate IR
    let ir = generate_ir(ast)
    
    // 3. Optimize
    let opt_ir = optimize(ir)
    
    // 4. Codegen
    let asm = codegen(opt_ir)
    
    // 5. Write assembly
    write_file(source + ".asm", asm)
    
    // 6. Assemble
    system_call("ml64 /c /Fo " + source + ".obj " + source + ".asm")
    
    // 7. Link
    link_program(source + ".obj", output)
    
    return 0
}
```

3. **Error Handling** (1h)
4. **Testing** (2h)

### Week 3: Self-Hosting & Testing (8-10h)

#### Day 1-2: Bootstrap Stage 1 (4-5h)

**Tasks**:
1. **Compile v6.2 with v5.0** (2h)
```powershell
# Alle v6.2 source files
$sources = Get-ChildItem ssl-v6\src\*.ssl -Recurse

# Mit v5.0 zu AST kompilieren, dann zu native
# (benötigt v5.0 extension)
```

2. **Manual Linking** (1h)
3. **Create Stage 1 Binary** (1h)
4. **Test Stage 1** (1h)

#### Day 3: Bootstrap Stage 2 (3-4h)

**Tasks**:
1. **Compile v6.2 with v6.2 (Stage 1)** (2h)
```powershell
.\ssl-v6-stage1.exe build ssl-v6\src\main.ssl -o ssl-v6-stage2.exe
```

2. **Hash Verification** (1h)
```powershell
$hash1 = (Get-FileHash ssl-v6-stage1.exe).Hash
$hash2 = (Get-FileHash ssl-v6-stage2.exe).Hash

if ($hash1 -eq $hash2) {
    Write-Output "✅ SELF-HOSTING SUCCESSFUL!"
}
```

3. **Testing** (1h)

#### Day 4: Testing & Validation (3-4h)

**Tasks**:
1. **Test Suite** (2h)
   - Compile 20+ test programs
   - Verify outputs
   - Performance benchmarks

2. **Bug Fixes** (1h)
3. **Documentation** (1h)

## Detailed Task Checklist

### Frontend (Week 1)
- [ ] Lexer: scan_number()
- [ ] Lexer: scan_string()
- [ ] Lexer: scan_ident()
- [ ] Lexer: scan_operator()
- [ ] Parser: parse_expression() (Pratt)
- [ ] Parser: parse_statement()
- [ ] Parser: parse_declaration()
- [ ] Parser: error_recovery()
- [ ] IR: gen_expr_ir()
- [ ] IR: gen_stmt_ir()
- [ ] IR: SSA construction

### Backend (Week 2)
- [ ] Codegen: emit_binary_ops()
- [ ] Codegen: emit_if()
- [ ] Codegen: emit_while()
- [ ] Codegen: emit_call()
- [ ] Codegen: emit_var_access()
- [ ] Runtime: Build ssl_runtime.lib
- [ ] Linker: integrate_linker()
- [ ] Build: cmd_build()

### Testing (Week 3)
- [ ] Bootstrap Stage 1
- [ ] Bootstrap Stage 2
- [ ] Hash verification
- [ ] Test suite (20+ programs)
- [ ] Performance benchmarks

## Success Metrics

- [ ] Stage 1 compiles successfully
- [ ] Stage 2 compiles successfully
- [ ] hash(Stage1) == hash(Stage2)
- [ ] All test programs compile & run
- [ ] Performance >= v5.0 (interpreter mode)

## Estimated Timeline

| Week | Hours | Tasks | Completion |
|------|-------|-------|------------|
| 1 | 16-20 | Frontend | 40% |
| 2 | 16-20 | Backend & Runtime | 80% |
| 3 | 8-10 | Self-Hosting & Testing | 100% |
| **Total** | **40-50** | **Complete** | **100%** |

---

**Created**: 2025-12-07 13:55  
**Status**: Implementation Roadmap Complete  
**Ready**: After Toolchain Setup
