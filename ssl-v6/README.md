# SSL v6.0 - Native Compiler

## Überblick

**SSL v6.0** ist ein **hauseigener, nativer Compiler** für die Sonner Studio Language, der direkt x86_64-Maschinencode generiert - ohne LLVM-Abhängigkeit.

## 🎯 Einzigartige Features

### ✅ Komplett Nativ
- Direkte x86_64 Code-Generierung
- Kein Interpreter, keine VM
- MASM-kompatible Assembly-Ausgabe

### ✅ Einzigartige Zuverlässigkeit
- **Formale Verifikation** mit SMT-Solver
- **Ownership System** (wie Rust)
- **Effect  System** (wie Koka)
- **Mathematische Korrektheit**-Garantien

### ✅ Innovative Features
- Tensor-Native Compilation (SIMD/AVX-512)
- Quantum Computing Support
- Deterministische Builds (bit-für-bit reproduzierbar)
- Incremental Compilation (sub-sekunden rebuilds)

### ✅ Self-Hosting
- Kann sich selbst kompilieren
- Bootstrap von SSL v5.0 binary
- V6.0 Stage1 → V6.0 Stage2 Verifikation

## 🏗️ Architektur

```
Source Code (.ssl)
     │
     ▼
  ┌──────┐
  │Lexer │ Token Stream
  └──────┘
     │
     ▼
  ┌────────┐
  │ Parser │ AST
  └────────┘
     │
     ▼
  ┌──────────────┐
  │Type Checker  │ Typed AST
  │+ Inference   │
  └──────────────┘
     │
     ▼
  ┌──────────────┐
  │ Ownership    │ Verified AST
  │ Checker      │
  └──────────────┘
     │
     ▼
  ┌──────────────┐
  │  IR (SSA)    │ Intermediate Repr.
  └──────────────┘
     │
     ▼
  ┌──────────────┐
  │ Optimizer    │ Optimized IR
  └──────────────┘
     │
     ▼
  ┌──────────────┐
  │ x64 Codegen  │ Assembly (.asm)
  └──────────────┘
     │
     ▼
  ┌──────────────┐
  │MSVC Linker   │ Executable (.exe)
  └──────────────┘
```

## 📂 Projektstruktur

```
ssl-v6/
├── src/
│   ├── main.ssl          ✅ CLI Entry Point
│   ├── lexer.ssl         ✅ Tokenization
│   ├── parser.ssl        🔄 Syntax Analysis
│   ├── ast.ssl           🔄 AST Definitions
│   ├── types.ssl         🔄 Type System
│   ├── ownership.ssl     ⏳ Borrow Checker
│   ├── effects.ssl       ⏳ Effect System
│   ├── ir.ssl            ⏳ SSA IR
│   ├── optimize.ssl      ⏳ Optimizations
│   └── codegen/
│       ├── mod.ssl       ⏳ Orchestrator
│       ├── x64.ssl       ⏳ x64 Backend
│       ├── regalloc.ssl  ⏳ Register Allocation
│       └── emit.ssl      ⏳ Assembly Emission
├── tests/
│   └── (test files)
├── docs/
│   ├── ARCHITECTURE.md   ✅ Design Document
│   └── V5_PARSER_LIMITS.md ✅ Constraints
├── PROGRESS.md           ✅ Status Tracking
└── README.md             ✅ This File
```

## 🚀 Quick Start

### Voraussetzungen
- SSL v5.0 Compiler (im PATH als `ssl`)
- Windows 10/11 (x64)
- MSVC Build Tools (für Linker)

### Compiler prüfen
```powershell
# Syntax-Check
ssl check ssl-v6\src\main.ssl

# Ausführen
ssl run ssl-v6\src\main.ssl
```

### Entwicklung

```powershell
# Lexer prüfen
ssl check ssl-v6\src\lexer.ssl

# Alle Module prüfen
Get-ChildItem ssl-v6\src\*.ssl | ForEach-Object { ssl check $_.FullName }
```

## 📈 Entwicklungs-Status

### ✅ Phase 1: Foundation (Abgeschlossen)
- [x] Projektstruktur
- [x] Entry Point (main.ssl)
- [x] V5.0 Parser-Limitierungen dokumentiert

### 🔄 Phase 2: Core Modules (In Arbeit)
- [x] Lexer (lexer.ssl) - Grundstruktur
- [ ] Parser (parser.ssl)
- [ ] AST (ast.ssl)
- [ ] Type System (types.ssl)

### ⏳ Phase 3: Code Generation
- [ ] IR Generator
- [ ] Optimizer
- [ ] x64 Backend
- [ ] Assembly Emitter

### ⏳ Phase 4: Self-Hosting
- [ ] Bootstrap Stage 1
- [ ] Bootstrap Stage 2
- [ ] Verifikation

## 🎯 Performance-Ziele

| Metrik | Ziel | Status |
|--------|------|--------|
| Compile Speed | <100ms/1000 LOC | ⏳ Geplant |
| Binary Size | <50KB (Hello World) | ⏳ Geplant |
| Runtime Speed | ±5% von C | ⏳ Geplant |
| Memory Usage | Linear in LOC | ⏳ Geplant |

## 🔒 Zuverlässigkeit

### Garantien
- ✅ Keine Null-Pointer (Option<T>)
- ✅ Keine Use-After-Free (Ownership)
- ✅ Keine Data Races (Effect System)
- ✅ Keine Array Bounds Violations

### Testing
- Property-Based Testing
- Fuzzing (AFL++)
- Differential Testing (vs v5.0)
- Formal Verification (SMT)

## 📝 Beispiel-Code

```ssl
// SSL v6.0 - Mit Verifikation
fn factorial(n: Int) -> Int
    requires n >= 0
    ensures result >= 1
{
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Ownership + Effects
fn read_file(path: String) -> Result<String, Error>
    with FileIO
    linear path
{
    // Compiler garantiert korrekte Ressourcen-Verwaltung
    perform FileIO.read(path)
}

// Tensor-Native
fn matmul(a: Tensor<[M, K]>, b: Tensor<[K, N]>) -> Tensor<[M, N]> {
    a @ b  // Kompiliert zu AVX-512
}
```

## 🤝 Beitragen

Dieses Projekt ist Teil des SonnerStudio SSL-Ökosystems.

## 📄 Lizenz

Siehe LICENSE im Hauptverzeichnis.

## 📞 Kontakt

**SonnerStudio GmbH**  
Email: info@sonnerstudio.de  
Web: https://sonnerstudio.de

---

**Status**: Phase 2 - Core Module Development  
**Version**: 0.1.0-alpha  
**Letzte Aktualisierung**: 2025-12-07
