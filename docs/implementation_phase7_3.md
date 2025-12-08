// Phase 7.3: Distributed Computing
// Implementation details and code changes for remote execution via spawn on

## Overview
This document describes the implementation of distributed computing features in SSL, allowing code execution on remote `ssld` daemon instances.

## Architecture
- **Local SSL VM**: Connects to remote ssld daemons via TCP
- **ssld Daemon**: Network server listening on TCP port 8080
- **Protocol**: JSON-serialized AST transmitted over TCP
- **Communication**: Fire-and-forget model (no return values in MVP)

## Core Components

### 1. NetworkChannel Type
Added `Value::NetworkChannel(String)` to represent network connections:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    // ... existing variants
    NetworkChannel(String), // TCP address like "127.0.0.1:8080"
}
```

### 2. AST Serialization
All AST types now derive `Serialize` and `Deserialize` from serde:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    // ... all variants can be serialized to JSON
    SpawnOn { address: String, block: Vec<Statement> },
}
```

### 3. spawn on Syntax
New syntax for remote execution:
```ssl
spawn on "127.0.0.1:8080" {
    print("Executing remotely!")
}
```

Parser recognizes `spawn on "address" { ... }` and creates `Statement::SpawnOn`.

### 4. ssld Network Server
Reimplemented `src/bin/ssld.rs` as TCP server:
- Listens on `0.0.0.0:8080`
- Accepts connections and reads JSON-serialized AST
- Executes received code in isolated interpreter
- Handles errors gracefully

### 5. Remote Execution Logic
In `src/interpreter.rs`, `Statement::SpawnOn` handler:
- Serializes block to JSON
- Opens TCP connection to target address  
- Sends AST and closes connection
- Runs in background thread (fire-and-forget)

## Testing
Created `examples/distributed_demo.ssl` demonstrating localhost spawn:
```ssl
spawn on "127.0.0.1:8080" {
    print("Remote: Hello from ssld!")
}
```

## MVP Simplifications
- **Localhost only**: Production would support TLS and authentication
- **No return values**: Fire-and-forget model for simplicity
- **JSON Protocol**: Binary protocols would be more efficient
- **Synchronous**: No async/await in current implementation

## Next Steps
- Test with actual ssld daemon running
- Add network error handling
- Consider return value channel
- Implement authentication/TLS for production
