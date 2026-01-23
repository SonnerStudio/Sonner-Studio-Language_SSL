# Runtime Integration Plan

## Ziel

Runtime Library (memory.asm, string.asm, list.asm) mit v6.0 Compiler integrieren.

## Integration Steps

### Step 1: SSL Builtin Interface ✅ (Nächster Schritt)
Erstelle `src/builtins.ssl` mit Funktionen, die Runtime aufrufen.

### Step 2: Codegen Integration
Erweitere `codegen/x64.ssl` zum Generieren von Runtime-Calls.

### Step 3: Test Integration
Erstelle SSL-Programme, die Builtins nutzen.

### Step 4: Linker Setup
Stelle sicher, dass Runtime-Objekte gelinkt werden.

## Step 1 Details: Builtin Interface

### Memory Builtins
```ssl
// src/runtime_builtins.ssl

// Memory allocation - calls ssl_malloc
fn builtin_malloc(size: Int) -> Any {
    // Generates: call ssl_malloc
    asm_inline("ssl_malloc", size)
}

// Memory deallocation - calls ssl_free
fn builtin_free(ptr: Any) -> Int {
    asm_inline("ssl_free", ptr)
}
```

### String Builtins
```ssl
// String length - calls ssl_string_length
fn builtin_string_length(s: String) -> Int {
    asm_inline("ssl_string_length", s)
}

// String concatenation - calls ssl_string_concat
fn builtin_string_concat(a: String, b: String) -> String {
    asm_inline("ssl_string_concat", a, b)
}

// Character at index - calls ssl_string_char_at
fn builtin_char_at(s: String, i: Int) -> String {
    asm_inline("ssl_string_char_at", s, i)
}

// Substring - calls ssl_string_substring
fn builtin_substring(s: String, start: Int, end: Int) -> String {
    asm_inline("ssl_string_substring", s, start, end)
}
```

### List Builtins
```ssl
// Create list - calls ssl_list_create
fn builtin_list_create(elem_size: Int, capacity: Int) -> Any {
    asm_inline("ssl_list_create", elem_size, capacity)
}

// List append - calls ssl_list_append
fn builtin_list_append(lst: Any, item: Any) -> Int {
    asm_inline("ssl_list_append", lst, item)
}

// List get - calls ssl_list_get
fn builtin_list_get(lst: Any, index: Int) -> Any {
    asm_inline("ssl_list_get", lst, index)
}
```

## Approach: Simplified Implementation

Da `asm_inline` noch nicht existiert, verwenden wir einen pragmatischeren Ansatz:

### Codegen generiert direkt Runtime-Calls

```ssl
// Enhanced codegen/x64.ssl

fn emit_builtin_call(name: String, args: Any) -> String {
    if name == "string_length" {
        emit_string_length_call(args)
    } else if name == "string_concat" {
        emit_string_concat_call(args)
    } else if name == "malloc" {
        emit_malloc_call(args)
    }
    // ... etc
}

fn emit_string_length_call(args: Any) -> String {
    // Assume arg is in rax (previous expression result)
    let s1 = "    mov rcx, rax\n"
    let s2 = "    call ssl_string_length\n"
    s1 + s2
}
```

## Next Immediate Steps

1. ✅ Create integration plan (this file)
2. 🔄 Create simplified runtime_builtins.ssl
3. ⏳ Update codegen to recognize builtins
4. ⏳ Create test program using runtime
5. ⏳ Document linking process

---

**Status**: Step 1 in progress  
**Created**: 2025-12-07 13:35
