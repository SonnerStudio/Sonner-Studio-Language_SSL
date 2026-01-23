# SSL v5.0 Parser Limitations

## Critical Constraints for v6.0 Source Code

Based on extensive testing, the following limitations have been identified in the SSL v5.0 parser:

### ❌ NOT SUPPORTED

1. **Method Call Syntax**
   - Cannot use: `list.length()`
   - Must use: `length(list)`
   
2. **Generic Type Annotations**
   - Cannot use: `List<String>`, `Map<K, V>`
   - Must use: `List`, `Map` (untyped)
   
3. **Namespace/Path Syntax**
   - Cannot use: `std::env::args()`
   - Must use: `env_args()` (direct function call)
   
4. **Blocks in Match Arms**
   - Cannot use:
     ```ssl
     match x {
         "a" => { println("A") }
         _ => { println("other") }
     }
     ```
   - Parser interprets `{`  after `=>` as map literal!
   
5. **Trait Bounds in Type Signatures**
   - Cannot use: `<K: Hash, V>`
   - Must use: `<K, V>` (but generics don't work anyway)
   
6. **Tuple Types in Generics**
   - Cannot use: `List<(K, V)>`
   - Parser fails with "Expected LBracket, got Ok(Lt)"
   
7. **Else-If Chains**
   - Cannot use: `} else if x {`
   - Must use nested: `} else { if x { } }`
   
8. **String Format Syntax**
   - Cannot use: `println("value: {}", x)`
   - Must use: `println("value: " + x)`

### ✅ SUPPORTED

1. **Basic Function Definitions**
   - `fn name(param: Type) -> RetType { }`
   - Type annotations ARE REQUIRED for parameters and return types
   
2. **Match with Simple Expressions**
   - `match x { 1 => expr, _ => expr }`
   - Arms must be simple expressions, not blocks
   
3. **Simple If-Else**
   - `if cond { } else { }`
   - Single-level if-else works fine
   
4. **String Concatenation**
   - `"hello" + " " + "world"`
   
5. **List Literals**
   - `[1, 2, 3]`, `["a", "b"]`
   
6. **Nested Functions**
   - Functions can call other functions
   
7. **While Loops**
   - `while i < 10 { i = i + 1 }`

## Workarounds for v6.0

To write v6.0 source code that v5.0 can parse:

1. **Use function-style calls everywhere**: `length(list)` not `list.length()`
2. **Avoid type annotations with generics**: Use `List` not `List<String>`
3. **Break complex match into functions**: Each match case calls a separate function
4. **Use nested if-else instead of else-if chains**
5. **All functions need type annotations**: `fn f(x: Int) -> String`
6. **Use `+` for string concatenation only**
7. **Never use `{` blocks in match arms**

## Example: Valid v5.0-Compatible Code

```ssl
fn main() {
    let args = env_args()
    if length(args) > 0 {
        process_args(args)
    } else {
        print_usage()
    }
}

fn process_args(args) {
    let cmd = args[0]
    if cmd == "build" {
        do_build(args)
    } else {
        if cmd == "run" {
            do_run(args)
        } else {
            println("Unknown: " + cmd)
        }
    }
}

fn length(list) -> Int {
    // Builtin
    0
}

fn env_args() -> List {
    []
}
```

This is valid SSL v5.0 code that will parse correctly!
