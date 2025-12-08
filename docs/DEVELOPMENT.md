# SSL - Entwicklungsdokumentation

**Sonner Studio Language (SSL) - Technical Development Guide**

Version: 0.1.0  
Datum: November 2025  
Autor: Sonner Studio Team

---

## 1. Architektur-Übersicht

### 1.1 Compiler-Pipeline

```
Source Code (.ssl)
    ↓
Lexer (Logos) → Tokens
    ↓
Parser → Abstract Syntax Tree (AST)
    ↓
Interpreter → Execution
```

**Komponenten:**
- **Lexer**: `src/lexer.rs` - Token-Generierung mit Logos
- **Parser**: `src/parser.rs` - Recursive Descent Parser
- **AST**: `src/ast.rs` - Typdefinitionen für Syntax-Baum
- **Interpreter**: `src/interpreter.rs` - Tree-Walking Interpreter
- **Stdlib**: `src/stdlib/` - Native Funktionen (fs, http, json, env)

### 1.2 Typsystem

**Hybrid-Ansatz:**
- Statische Typen mit Annotationen (`fn foo(x: Int) -> String`)
- Dynamische Typen mit Inferenz (`let x = 42`)
- Runtime Type Checking

**Value-Typen:**
```rust
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),  // Neu in v0.1.0
    Function { params, body },
    Qubit(usize),
    Nil,
}
```

---

## 2. Quantum Computing Implementation

### 2.1 State Vector Simulator

**Datei:** `src/interpreter.rs` (QuantumState struct)

**Implementierung:**
```rust
struct QuantumState {
    state: Vec<Complex<f64>>,
    num_qubits: usize,
}
```

**Operationen:**
- **Qubit()**: Initialisiert |0⟩ (state = [1, 0])
- **H(q)**: Hadamard-Gate (Superposition)
- **X(q)**: Pauli-X (NOT)
- **CNOT(control, target)**: Controlled-NOT
- **Measure(q)**: Wavefunction Collapse

**Beispiel:**
```ssl
let q = Qubit()      // |0⟩
H(q)                 // 1/√2(|0⟩ + |1⟩)
let result = Measure(q)  // 0 oder 1 (50/50)
```

### 2.2 Limitierungen

- Simuliert 8-10 Qubits (2^n Zustandsvektoren)
- Keine Quantenfehlerkorrektur
- Perfekte Messung (kein Rauschen)

---

## 3. Concurrency Model

### 3.1 CSP-Style Channels

**Implementierung:**
```rust
use std::sync::mpsc::{channel, Sender, Receiver};

Value::ChannelSender(Sender<Value>)
Value::ChannelReceiver(Receiver<Value>)
```

**API:**
```ssl
let chan = channel()       // Returns [sender, receiver]
send(chan[0], value)       // Blocking send
let val = recv(chan[1])    // Blocking receive
```

### 3.2 Thread Spawning

**Lokal:**
```ssl
spawn {
    // Läuft in separatem Thread
    compute()
}
```

**Distributed:**
```ssl
spawn on "host:port" {
    // Läuft auf Remote-Maschine via ssld
    remote_compute()
}
```

**Implementierung:** Clone Environment + QuantumState, neue Thread via `std::thread::spawn`

---

## 4. Standard Library

### 4.1 Module-Struktur

```
src/stdlib/
├── mod.rs          # register_all()
├── fs.rs           # File System
├── http.rs         # HTTP Client
├── json.rs         # JSON Parse/Stringify
└── env.rs          # Environment
```

### 4.2 Native Function Registration

**Pattern:**
```rust
pub fn register(interpreter: &mut Interpreter) {
    interpreter.register_native_function("function_name", |args| {
        // Validierung
        if args.len() != expected {
            return Err("Wrong arity");
        }
        
        // Implementierung
        match &args[0] {
            Value::String(s) => {
                // Logic
                Ok(Value::String(result))
            }
            _ => Err("Type error")
        }
    });
}
```

### 4.3 JSON Implementation

**Maps → JSON Objects:**
```rust
// SSL
let data = { "name": "SSL", "version": "0.1.0" }

// Intern: Value::Map(HashMap)
// JSON: {"name":"SSL","version":"0.1.0"}
```

**Bidirektional:**
- `json_parse()`: String → Value::Map
- `json_stringify()`: Value::Map → String

---

## 5. Parser Details

### 5.1 Expression Grammar

```ebnf
expression    := binary_op | primary
binary_op     := primary OP primary
primary       := literal | identifier | function_call | list | map | index
literal       := INT | FLOAT | STRING | BOOL
list          := "[" (expression ("," expression)*)? "]"
map           := "{" (STRING ":" expression ("," STRING ":" expression)*)? "}"
function_call := IDENTIFIER "(" (expression ("," expression)*)? ")"
index         := primary "[" expression "]"
```

### 5.2 Map Parsing

**Neu in v0.1.0:**
```rust
fn map_literal(&mut self) -> Result<Expression> {
    self.consume(Token::LBrace)?;
    let mut pairs = Vec::new();
    
    while !self.check(Token::RBrace) {
        let key = self.expect_string_literal()?;
        self.consume(Token::Colon)?;
        let value = self.expression()?;
        pairs.push((key, value));
        
        if !self.match_token(Token::Comma) {
            break;
        }
    }
    
    self.consume(Token::RBrace)?;
    Ok(Expression::MapLiteral(pairs))
}
```

---

## 6. Error Handling

### 6.1 Try/Recover

**Syntax:**
```ssl
try {
    risky_operation()
} recover (err) {
    print("Error:", err)
}
```

**Implementierung:**
```rust
Statement::TryRecover { try_block, error_var, recover_block } => {
    match self.interpret(try_block) {
        Ok(val) => Ok(val),
        Err(e) => {
            // Bind error to variable
            self.env.define(error_var, Value::String(e));
            self.interpret(recover_block)
        }
    }
}
```

### 6.2 AI-Integration (Geplant)

Future: Fehleranalyse via LLM, Auto-Recovery-Vorschläge

---

## 7. Tools & Infrastruktur

### 7.1 CLI (ssl)

**Commands:**
```bash
ssl run <file>      # Interpret & Execute
ssl check <file>    # Syntax Check
ssl doctor          # System Diagnostics
ssl lsp             # Language Server
```

### 7.2 AI Daemon (ssld)

**Zweck:** Distributed Computing Backend

**Workflow:**
1. `ssld` lauscht auf Port 8080
2. Client sendet serialisierten AST
3. `ssld` interpretiert remote
4. Ergebnis zurück an Client

**Serialisierung:** Serde JSON

### 7.3 VS Code Extension

**Pfad:** `editors/vscode/`

**Features:**
- Syntax Highlighting (TextMate Grammar)
- Snippets
- Language Configuration

---

## 8. Build & Test

### 8.1 Dependencies

**Cargo.toml:**
```toml
[dependencies]
logos = "0.13"           # Lexer
serde = { features = ["derive"] }
serde_json = "1.0"       # JSON
reqwest = { features = ["blocking"] }  # HTTP
tokio = { features = ["full"] }        # Async (LSP)
num = "0.4"              # Quantum Math
```

### 8.2 Testing

**Unit Tests:**
```bash
cargo test
```

**Integration Tests:**
```bash
cargo run --bin ssl -- run examples/quantum_random.ssl
cargo run --bin ssl -- run examples/web_api.ssl
```

---

## 9. Roadmap

### v0.2.0
- [ ] Variable Reassignment (`mut`)
- [ ] Bessere Fehlermeldungen
- [ ] String-Operationen
- [ ] List-Comprehensions

### v0.5.0
- [ ] JIT Compilation (LLVM Backend)
- [ ] Package Manager
- [ ] Async/Await
- [ ] HTTP Server

### v1.0.0
- [ ] Production-Ready
- [ ] Stable API
- [ ] Performance Optimizations
- [ ] Enterprise Features

---

## 10. Contributing

**Wichtige Dateien:**
- `CONTRIBUTING.md` - Contribution Guidelines
- `GOVERNANCE.md` - Decision Process
- `docs/rfcs/` - Design Proposals

**Workflow:**
1. Fork Repository
2. Feature Branch erstellen
3. Tests hinzufügen
4. PR erstellen
5. RFC für große Features

---

## 11. Lizenz

Dual Licensed: **MIT** und **Apache 2.0**

Nutzer können wählen!

---

**Kontakt:** GitHub Issues & Discussions  
**Dokumentation:** https://github.com/SonnerStudio/Sonner-Studio-Language_SSL
