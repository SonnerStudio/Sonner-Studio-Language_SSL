# SSL v6.2 - Code Standardization Guide

## Patterns from v6.0/v6.1

### 1. File Structure (Standard)
```ssl
// Module Header
// =============================================================================
// MODULE: NAME
// =============================================================================
// Purpose: Description
// Input:   Input description  
// Output:  Output description
// =============================================================================

// Constants
fn CONST_NAME() -> Type { value }

// Helper Functions
fn helper_function() -> Type { }

// Main Functions
fn main_function() -> Type { }

// Module Main (Testing)
fn main() { }
```

### 2. Naming Conventions
- **Functions**: `snake_case` (e.g., `parse_expression`, `emit_code`)
- **Constants**: `UPPER_SNAKE_CASE` via functions (e.g., `TOKEN_INT()`)
- **Variables**: `snake_case` (e.g., `token_count`, `is_valid`)
- **Modules**: Lowercase (e.g., `lexer.ssl`, `parser.ssl`)

### 3. Comment Patterns
- Section headers: `// ========` (79 chars)
- Subsections: `// --------` (79 chars)
- Inline: `// Description`
- Block: Used for detailed explanations

### 4. Function Signatures
```ssl
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // Implementation
}
```

### 5. Error Handling Pattern
```ssl
if error_condition {
    println("ERROR: Description")
    return error_value
}
```

### 6. Module Organization
1. Header comment
2. Constants/Config
3. Utility functions
4. Core functions
5. Main entry point
6. Test main()

## Applied to v6.2

### Standardized Files
- ✅ `lexer_standard.ssl` - Following v6 patterns
- ⏳ `parser_standard.ssl` - Next
- ⏳ `ir_standard.ssl` - Next
- ⏳ `codegen_standard.ssl` - Next

### Benefits
- Consistent code style
- Easier maintenance
- Clear structure
- Professional quality

---

**Status**: Standardization In Progress  
**Target**: All v6.2 modules homogenized
