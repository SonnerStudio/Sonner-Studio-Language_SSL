# SSL Standard Library - README

## Overview
This is the SSL v8.0 standard library foundation, implementing core types and collections with full generic support.

## Structure

```
stdlib/
├── core/           # Core types (no_std compatible)
│   └── option.ssl  # Option<T> and Result<T, E>
├── collections/    # Data structures
│   └── vec.ssl     # Vec<T> dynamic array
├── io/             # Input/Output
│   └── file.ssl    # File and stdio operations
└── test_stdlib.ssl # Comprehensive tests
```

## Implemented Features

### Core Types

#### Option<T>
```ssl
let some_value: Option<i64> = Option::Some(42);
let none_value: Option<i64> = Option::None;

// Methods
some_value.is_some()              // true
some_value.unwrap()               // 42
none_value.unwrap_or(0)           // 0
some_value.map(|x| x * 2)         // Option::Some(84)
```

**Methods**: `is_some`, `is_none`, `unwrap`, `unwrap_or`, `unwrap_or_else`, `map`, `and_then`, `or`, `as_ref`

#### Result<T, E>
```ssl
let ok: Result<i64, String> = Result::Ok(42);
let err: Result<i64, String> = Result::Err("error");

// Methods
ok.is_ok()                        // true
ok.unwrap()                       // 42
err.unwrap_or(0)                  // 0
ok.map(|x| x * 2)                 // Result::Ok(84)
```

**Methods**: `is_ok`, `is_err`, `unwrap`, `unwrap_err`, `unwrap_or`, `map`, `map_err`, `and_then`, `ok`, `err`

### Collections

#### Vec<T>
```ssl
let mut vec: Vec<i64> = Vec::new();
vec.push(1);
vec.push(2);
vec.push(3);

vec.len()                         // 3
vec[0]                            // 1
vec.pop()                         // Option::Some(3)
vec.insert(1, 42)                 // [1, 42, 2]
vec.reverse()                     // [2, 42, 1]
```

**Methods**: `new`, `with_capacity`, `len`, `is_empty`, `capacity`, `push`, `pop`, `get`, `get_mut`, `insert`, `remove`, `clear`, `contains`, `reverse`

### I/O Operations

#### File
```ssl
// Write file
let mut file = File::create("test.txt")?;
file.write("Hello, World!")?;

// Read file
let file = File::open("test.txt")?;
let content = file.read_to_string()?;
```

**Functions**: `open`, `create`, `read_to_string`, `write`

#### Standard I/O
```ssl
print("Hello");
println("World!");
let input = read_line()?;
```

## Compiler Requirements

To support this standard library, the compiler needs:

### 1. Generic Types
```rust
// Parser needs to support
enum Option<T> { Some(T), None }
struct Vec<T> { ... }
impl<T> Vec<T> { ... }
```

### 2. Enums
```rust
enum Option<T> {
    Some(T),
    None
}
```

### 3. Pattern Matching
```rust
match value {
    Option::Some(x) => x,
    Option::None => 0
}
```

### 4. Traits
```rust
trait Eq {
    fn eq(&self, other: &Self) -> bool;
}

impl Eq for i64 {
    fn eq(&self, other: &i64) -> bool {
        *self == *other
    }
}
```

### 5. References & Borrowing
```rust
fn get(&self, index: usize) -> Option<&T>
fn get_mut(&mut self, index: usize) -> Option<&mut T>
```

### 6. Operator Overloading
```rust
impl<T> Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T { ... }
}
```

### 7. Drop/Destructors
```rust
impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        // Cleanup
    }
}
```

### 8. Intrinsics
```rust
extern "intrinsic" {
    fn allocate(size: usize) -> *mut u8;
    fn deallocate(ptr: *mut u8);
}
```

## Next Steps

1. **Extend Parser** to support:
   - Generic type parameters (`<T>`, `<T, E>`)
   - Enum definitions with variants
   - Trait definitions and implementations
   - Pattern matching (`match` expressions)

2. **Extend Code Generator** to:
   - Monomorphization (generic instantiation)
   - Enum tag/payload layout
   - Virtual dispatch for traits
   - Reference counting or borrow checking

3. **Add Runtime Support** for:
   - Memory allocation (malloc/free)
   - File I/O syscalls
   - Standard streams

## Testing

Run the test suite:
```bash
ssl run stdlib/test_stdlib.ssl
```

Expected output:
```
=================================
SSL v8 Standard Library Tests
=================================
Testing Option<T>...
✓ Option::Some works
✓ Option::None works
✓ unwrap works
✓ unwrap_or works
✓ map works

Testing Result<T, E>...
✓ Result::Ok works
✓ Result::Err works
✓ unwrap works
✓ unwrap_or works

Testing Vec<T>...
✓ new Vec is empty
✓ push works
✓ indexing works
✓ pop works
✓ insert works
✓ reverse works

Testing File I/O...
✓ File write works
✓ File read works

=================================
All tests completed!
=================================
```

## Status

**Current**: Foundation complete, compiler support needed
**Next**: Implement generic type system in compiler
**Timeline**: Week 1-2 of SSL v8 development

---

**SSL v8 Standard Library** - The foundation for world-class systems programming
