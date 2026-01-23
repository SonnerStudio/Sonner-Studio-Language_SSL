# Getting Started with SSL

**Your 5-Minute Guide to Quantum-Ready Programming**

---

## Installation

### Prerequisites
- Rust toolchain (https://rust-lang.org)
- Git

### Clone & Build
```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL
cd Sonner-Studio-Language_SSL
cargo build --release
```

---

## Your First SSL Program

Create `hello.ssl`:
```ssl
fn main() {
    print("Hello, SSL!")
}
```

Run it:
```bash
cargo run --bin ssl -- run hello.ssl
```

---

## Example 1: Quantum Random Number
```ssl
fn main() {
    let q = Qubit()
    H(q)  // Hadamard: superposition
    let result = Measure(q)
    print("Quantum bit:", result)  // 0 or 1 (50/50)
}
```

**What's happening:**
1. Create a qubit in |0⟩ state
2. Apply Hadamard gate (superposition)
3. Measure (collapses to 0 or 1)

---

## Example 2: Parallel Computing
```ssl
fn main() {
    let chan = channel()
    
    spawn {
        send(chan[0], "Hello from thread!")
    }
    
    let message = recv(chan[1])
    print(message)
}
```

**Key concepts:**
- `spawn`: Create parallel thread
- `channel()`: Communication channel
- `send/recv`: Message passing

---

## Example 3: Web API & JSON
```ssl
fn main() {
    try {
        // Fetch JSON from API
        let response = http_get("https://api.github.com/users/github")
        let data = json_parse(response)
        
        // Save to file
        fs_write("github.json", json_stringify(data))
        print("Data saved!")
        
    } recover (err) {
        print("Error:", err)
    }
}
```

**Features used:**
- HTTP client (`http_get`)
- JSON parsing (`json_parse`)
- File I/O (`fs_write`)
- Error handling (`try/recover`)

---

## Example 4: Maps & Data
```ssl
fn main() {
    let config = {
        "name": "MyApp",
        "version": "1.0.0",
        "features": ["quantum", "parallel", "distributed"]
    }
    
    print("App:", config)
    print("JSON:", json_stringify(config))
}
```

---

## What's Next?

### Explore Examples
```bash
cd examples/
ls *.ssl
```

### Read Documentation
- [Language Guide](../DOCUMENTATION.md)
- [Standard Library](../stdlib.md)
- [Examples](../EXAMPLES.md)

### Join Community
- GitHub Discussions
- Submit Issues
- Contribute!

---

**You're ready to build with SSL!** 🚀⚛️
