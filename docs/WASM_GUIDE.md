# SSL WASM Guide

## WebAssembly Compilation

SSL v4.0 supports compiling to WebAssembly for running in browsers and other WASM environments.

## Quick Start

```bash
# Compile to WASM
ssl build --target wasm src/main.ssl

# Compile with JS bindings
ssl build --target wasm-js src/main.ssl

# Output files:
# - target/module.wasm
# - target/bindings.js (with --target wasm-js)
# - target/bindings.d.ts (TypeScript declarations)
# - target/index.html (demo page)
```

## Example

### SSL Source (main.ssl)

```ssl
@wasm_export
fn add(a: Int, b: Int) -> Int {
    a + b
}

@wasm_export
fn factorial(n: Int) -> Int {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

@wasm_export
fn fibonacci(n: Int) -> Int {
    match n {
        0 => 0
        1 => 1
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}
```

### JavaScript Usage

```javascript
import { add, factorial, fibonacci } from './bindings.js';

async function main() {
    const result = await add(2, 3);
    console.log(`2 + 3 = ${result}`);
    
    console.log(`10! = ${await factorial(10)}`);
    console.log(`fib(20) = ${await fibonacci(20)}`);
}

main();
```

### HTML Integration

```html
<!DOCTYPE html>
<html>
<head>
    <title>SSL WASM Demo</title>
</head>
<body>
    <h1>SSL WebAssembly</h1>
    <div id="result"></div>
    
    <script type="module">
        import init, { add } from './bindings.js';
        
        async function run() {
            await init();
            const result = add(10, 20);
            document.getElementById('result').textContent = `10 + 20 = ${result}`;
        }
        
        run();
    </script>
</body>
</html>
```

## WASM Attributes

### @wasm_export

Exports a function for use from JavaScript:

```ssl
@wasm_export
fn my_function(x: Int) -> Int {
    x * 2
}
```

### @wasm_import

Import a JavaScript function:

```ssl
@wasm_import(module: "env", name: "console_log")
fn log(msg: String)
```

## Memory Management

SSL uses linear memory for strings and arrays:

```ssl
// Strings are automatically converted
@wasm_export
fn greet(name: String) -> String {
    "Hello, ${name}!"
}
```

## JavaScript Interop

### Console API

```ssl
@wasm_js
fn console_log(msg: String) {
    // Maps to console.log()
}
```

### DOM API

```ssl
@wasm_js
fn get_element_by_id(id: String) -> Element {
    // Maps to document.getElementById()
}
```

### Fetch API

```ssl
@wasm_js
async fn fetch_json(url: String) -> Json {
    // Maps to fetch().then(r => r.json())
}
```

## Optimization

### Release Build

```bash
ssl build --target wasm --release src/main.ssl
```

Enables:
- Dead code elimination
- Function inlining
- Constant folding
- WASM optimization passes

### Size Reduction

```bash
# Use wasm-opt for further optimization
wasm-opt -Os target/module.wasm -o target/module.opt.wasm
```

## Browser Support

| Browser | Version | Status |
|---------|---------|--------|
| Chrome | 57+ | ✅ Full |
| Firefox | 52+ | ✅ Full |
| Safari | 11+ | ✅ Full |
| Edge | 16+ | ✅ Full |
| Node.js | 8+ | ✅ Full |
| Deno | 1.0+ | ✅ Full |

## Debugging

### Source Maps

```bash
ssl build --target wasm --debug src/main.ssl
```

Generates source maps for browser DevTools.

### Console Output

```ssl
@wasm_export
fn demo() {
    console.log("Debug message")
    console.error("Error message")
}
```

## Limitations

- No direct DOM manipulation (use JS interop)
- No threading (yet - SharedArrayBuffer planned)
- 4GB memory limit (WASM32)

## See Also

- [Edge Deployment](V4.0_FEATURES.md#3-edgeserverless-deployment) - Deploy WASM to edge
- [Mobile Guide](MOBILE_GUIDE.md) - WASM on mobile
- [Examples](../examples/) - More examples

---

*SSL v4.0.0 WASM Guide - December 2024*
