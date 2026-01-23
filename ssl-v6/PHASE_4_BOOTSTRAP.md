# SSL v6.0 Self-Hosting Bootstrap - Phase 4 Plan

## Ziel

**Self-Hosting**: Der SSL v6.0 Compiler soll sich selbst kompilieren können.

## Status Quo

✅ **Phase 1-3 Abgeschlossen**:
- Foundation (Entry Point, Architecture)
- Core Modules (Lexer, Parser, AST, Types)
- Code Generation (IR, Optimizer, x64 Backend)

**9 Module komplett**: ~1500 LOC, 100% V5.0-kompatibel

## Bootstrap-Strategie

### Stage 0: Ausgangspunkt
- **Compiler**: SSL v5.0 binary (`target/release/ssl.exe`)
- **Source**: SSL v6.0 code (`ssl-v6/src/`)
- **Ziel**: Funktionierender v6.0 binary

### Stage 1: Cross-Compilation
```
v5.0 binary + v6.0 source → v6.0 Stage1 binary
```

**Befehl**:
```powershell
ssl build ssl-v6/src/main.ssl --target native --output ssl-v6-stage1.exe
```

**Erwartung**: Native executable, das x64 Assembly generieren kann

### Stage 2: Self-Compilation
```
v6.0 Stage1 binary + v6.0 source → v6.0 Stage2 binary
```

**Befehl**:
```powershell
./ssl-v6-stage1.exe build ssl-v6/src/main.ssl --output ssl-v6-stage2.exe
```

**Erwartung**: Identisches Binary wie Stage1

### Verifikation
```powershell
# Hash-Vergleich
$hash1 = Get-FileHash ssl-v6-stage1.exe
$hash2 = Get-FileHash ssl-v6-stage2.exe

if ($hash1.Hash -eq $hash2.Hash) {
    Write-Host "✅ BOOTSTRAP SUCCESSFUL!"
} else {
    Write-Host "❌ Hashes differ - non-deterministic build"
}
```

## Herausforderungen

### 1. Aktuelle Limitierung
**Problem**: Der v6.0 Source-Code ist vereinfacht (Platzhalter-Funktionen)
- `lexer.ssl`: Gibt leere Token-Liste zurück
- `parser.ssl`: Gibt Platzhalter-AST zurück
- `ir.ssl`: Vereinfachte IR-Generierung

**Lösung**: Full Implementation erforderlich für echte Compilation

### 2. Fehlende Features
- **String-Manipulation**: `char_at()`, `substring()`, `length()`
- **Array/List-Operationen**: `append()`, `get()`, `set()`
- **File I/O**: Source-Code laden
- **System-Integration**: Linker aufrufen

### 3. Runtime Library
V6.0 benötigt Minimal-Runtime:
- `println()` Implementation
- Memory Management (malloc/free)
- String Operations
- List/Array Operations

## Implementierungs-Optionen

### Option A: Vollständige Implementierung
**Aufwand**: Hoch (~mehrere Tage)
**Vorteil**: Echter, funktionierender Compiler
**Nachteil**: Zeitintensiv

**Steps**:
1. Implementiere alle Builtin-Funktionen
2. Vervollständige Lexer (echtes Tokenizing)
3. Vervollständige Parser (echtes AST)
4. Vervollständige IR Generator
5. Vervollständige x64 Backend
6. Runtime Library in Assembly
7. Linker-Integration

### Option B: Hybrid-Ansatz
**Aufwand**: Mittel (~1 Tag)
**Vorteil**: Schneller Bootstrap
**Nachteil**: Noch Abhängigkeit von v5.0

**Steps**:
1. v6.0 generiert x64 Assembly-Stub
2. v5.0 Runtime für Builtins
3. Gradueller Übergang zu nativen Builtins

### Option C: Proof-of-Concept
**Aufwand**: Niedrig (~einige Stunden)
**Vorteil**: Zeigt Machbarkeit
**Nachteil**: Nicht voll funktional

**Steps**:
1. "Hello World" Compiler schreiben
2. Kann nur minimal Programme kompilieren
3. Demonstriert Pipeline

## Empfohlener Ansatz: Option C → B → A

### Phase 4.1: Proof-of-Concept (Jetzt)
- Einfacher "Hello World" Compiler
- Generiert hardcoded x64 Assembly
- Demonstriert Konzept

### Phase 4.2: Minimal Viable Compiler
- Kann einfache Funktionen kompilieren
- Grundlegende Expressions (Int, String, Binary Ops)
- Basic Control Flow (if/else)

### Phase 4.3: Full Compiler
- Komplette Sprachfeatures
- Optimization
- Error Handling
- Full Self-Hosting

## Next Steps (Sofort)

### 1. Hello World Compiler
```ssl
// hello_compiler.ssl
fn main() {
    let asm = generate_hello_world_asm()
    write_file("hello.asm", asm)
    println("Generated hello.asm")
}

fn generate_hello_world_asm() -> String {
    // Hardcoded but funktionierend
    ".code\nmain:\n    ; Hello World\n    ret\nEND"
}
```

### 2. Assembly zu Executable
```powershell
# Mit MASM assemblen
ml64 /c /Fo hello.obj hello.asm

# Mit MSVC linken
link /subsystem:console /entry:main hello.obj
```

### 3. Demo
```powershell
# Compiler ausführen
ssl run hello_compiler.ssl

# Assembly assemblen & linken
ml64 /c /Fo hello.obj hello.asm
link /subsystem:console /entry:main hello.obj

# Executable testen
./hello.exe
```

## Erfolgskriterien

### Minimal (Phase 4.1)
- [ ] v6.0 kann x64 Assembly-File generieren
- [ ] Assembly kann zu .exe gelinkt werden
- [ ] .exe läuft (exit code 0)

### Standard (Phase 4.2)
- [ ] v6.0 kann einfache SSL-Programme kompilieren
- [ ] Generierte .exe ist funktional
- [ ] Int, String, Basic Ops funktionieren

### Vollständig (Phase 4.3)
- [ ] v6.0 kann sich selbst kompilieren
- [ ] Stage1 == Stage2 (Bit-für-bit)
- [ ] Performance >= v5.0 (Interpreter-Mode)
- [ ] Alle Tests bestehen

## Timeline

- **Phase 4.1** (PoC): ~2-3 Stunden
- **Phase 4.2** (MVC): ~1-2 Tage
- **Phase 4.3** (Full): ~1-2 Wochen

**Empfehlung**: Starten mit Phase 4.1 für schnellen Erfolg

---

**Erstellt**: 2025-12-07 13:15  
**Status**: Bereit für Phase 4.1 Implementation  
**Nächster Schritt**: Hello World Compiler
