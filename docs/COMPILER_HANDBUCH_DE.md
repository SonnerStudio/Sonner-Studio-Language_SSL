# SSL v7.0 Compiler-Handbuch

Vollständiger Leitfaden zur SSL v7.0 Native Compiler-Architektur und Verwendung.

## Inhaltsverzeichnis

1. [Compiler-Übersicht](#compiler-übersicht)
2. [Kompilierungs-Pipeline](#kompilierungs-pipeline)
3. [Kommandozeilen-Interface](#kommandozeilen-interface)
4. [Optimierungsstufen](#optimierungsstufen)
5. [Cross-Compilation](#cross-compilation)
6. [Debugging](#debugging)
7. [Compiler-Interna](#compiler-interna)

---

## Compiler-Übersicht

SSL v7.0 verfügt über einen produktionsreifen nativen Compiler, der optimierten x64-Assembly-Code erzeugt.

### Hauptmerkmale

- **Native Kompilierung**: Direkte x64-Assembly-Ausgabe
- **Multi-Architektur**: x86_64, ARM64, Apple Silicon
- **AOT (Ahead-of-Time)**: Kein JIT, reiner nativer Code
- **SSA-basierte IR**: Moderne Zwischenrepräsentation
- **Multi-Pass-Optimizer**: Konstantenfaltung, DCE, Inlining
- **ABI-konform**: Windows x64, System V AMD64, AAPCS64

### Architektur

```
Quellcode (.ssl)
    ↓
Lexer → Tokens
    ↓
Parser → AST
    ↓
Typ-Prüfer → Typisierter AST
    ↓
IR-Generator → SSA IR
    ↓
Optimizer → Optimierte IR
    ↓
Code-Generator → Assembly (.asm)
    ↓
Assembler (NASM) → Object-Datei (.o)
    ↓
Linker (ld) → Ausführbare Datei
```

---

## Kompilierungs-Pipeline

### Phase 1: Lexikalische Analyse

**Eingabe**: Quellcode-Text  
**Ausgabe**: Token-Strom

```ssl
let x = 42 + 10
```

**Tokens**:
```
[LET, IDENTIFIER("x"), EQUALS, INT_LITERAL(42), PLUS, INT_LITERAL(10)]
```

### Phase 2: Parsing

**Eingabe**: Token-Strom  
**Ausgabe**: Abstrakter Syntaxbaum (AST)

**AST**:
```
VarDecl
├─ name: "x"
├─ type: Int
└─ value: BinaryOp
    ├─ op: PLUS
    ├─ left: 42
    └─ right: 10
```

### Phase 3: Typ-Prüfung

**Eingabe**: AST  
**Ausgabe**: Typisierter AST

Überprüft:
- Typ-Kompatibilität
- Funktionssignaturen
- Variablen-Gültigkeitsbereiche

### Phase 4: IR-Generierung

**Eingabe**: Typisierter AST  
**Ausgabe**: SSA-Form Zwischenrepräsentation

```
%0 = const 42
%1 = const 10
%2 = add %0, %1
store x, %2
```

### Phase 5: Optimierung

**Eingabe**: IR  
**Ausgabe**: Optimierte IR

Transformationen:
- Konstantenfaltung: `42 + 10` → `52`
- Eliminierung toten Codes
- Gemeinsame Teilausdrucks-Eliminierung

**Optimierte IR**:
```
%0 = const 52
store x, %0
```

### Phase 6: Code-Generierung

**Eingabe**: Optimierte IR  
**Ausgabe**: x64 Assembly

```nasm
section .text
global main

main:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    
    mov QWORD [rbp-8], 52  ; x = 52
    
    mov rax, 0
    add rsp, 32
    pop rbp
    ret
```

---

## Kommandozeilen-Interface

### Grundlegende Verwendung

```bash
# Kompilieren und ausführen
ssl run programm.ssl

# Zu ausführbarer Datei kompilieren
ssl compile programm.ssl

# Nur Syntax prüfen
ssl check programm.ssl

# Compiler-Version anzeigen
ssl --version

# Hilfe-Informationen
ssl --help
```

### Kompilierungs-Optionen

```bash
# Ausgabedatei angeben
ssl compile programm.ssl -o meinprogramm

# Nur Assembly ausgeben
ssl compile --emit asm programm.ssl

# IR ausgeben
ssl compile --emit ir programm.ssl

# Debug-Build
ssl compile --debug programm.ssl

# Release-Build (optimiert)
ssl compile --release programm.ssl
```

### Architektur-Auswahl

```bash
# Für spezifische Architektur kompilieren
ssl compile --arch x86_64 programm.ssl
ssl compile --arch arm64 programm.ssl
ssl compile --arch apple_m programm.ssl

# Target-Triple angeben
ssl compile --target x86_64-linux-gnu programm.ssl
ssl compile --target aarch64-linux-gnu programm.ssl
```

---

## Optimierungsstufen

### -O0 (Keine Optimierung)

Standard für Debug-Builds.

**Features**:
- Keine Optimierungen
- Schnelle Kompilierung
- Einfaches Debugging
- Alle Variablen erhalten

**Verwendung**: Entwicklung, Debugging

### -O1 (Basis-Optimierung)

Leichte Optimierungen mit minimalem Einfluss auf Kompilierzeit.

**Features**:
- Konstantenfaltung
- Einfache Eliminierung toten Codes
- Basis-Inlining

**Verwendung**: Schnelles Testen mit etwas Performance

### -O2 (Standard-Optimierung)

Standard für Release-Builds.

**Features**:
- Vollständige Konstanten-Propagierung
- Eliminierung toten Codes
- Schleifen-Optimierungen
- Inlining
- Register-Allokations-Optimierung

**Verwendung**: Produktions-Builds

### -O3 (Aggressive Optimierung)

Maximale Performance, längere Kompilierzeit.

**Features**:
- Alle -O2 Optimierungen
- Aggressives Inlining
- Vektorisierung (SIMD)
- Funktions-Spezialisierung

**Verwendung**: Performance-kritische Anwendungen

**Beispiel**:
```bash
ssl compile --release -O3 programm.ssl
```

---

## Cross-Compilation

### Setup

**ARM64 Toolchain installieren** (Linux/WSL):
```bash
sudo apt install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
```

### Cross-Compile für ARM64

```bash
# Für ARM64 von x86_64 kompilieren
ssl compile --arch arm64 programm.ssl

# Ausgabe-Dateien:
# - programm.asm (ARM64 Assembly)
# - programm.o (ARM64 Object-Datei)
# - programm (ARM64 Executable)
```

### Testen mit QEMU

```bash
# QEMU installieren
sudo apt install qemu-system-arm

# ARM64-Binary testen
qemu-aarch64 ./programm
```

### Cross-Compile für Apple Silicon

```bash
# Von x86_64 zu Apple M-Serie
ssl compile --arch apple_m --target aarch64-apple-darwin programm.ssl
```

---

## Debugging

### Debug-Symbole

```bash
# Mit Debug-Informationen kompilieren
ssl compile --debug programm.ssl
```

**Erzeugt**:
- DWARF Debug-Symbole
- Quellzeilen-Zuordnung
- Variablennamen erhalten

### Generiertes Assembly ansehen

```bash
ssl compile --emit asm programm.ssl
cat programm.asm
```

### IR ansehen

```bash
ssl compile --emit ir programm.ssl
cat programm.ir
```

### Verbose-Modus

```bash
ssl compile --verbose programm.ssl
```

**Ausgabe**:
```
[LEXER] Verarbeite Tokens...
[PARSER] Erzeuge AST...
[TYPECHECK] Prüfe Typen...
[IR] Erzeuge Zwischenrepräsentation...
[OPTIMIZE] Führe Optimierungen aus (Pass 1/3)...
[CODEGEN] Erzeuge x64 Assembly...
[ASSEMBLE] Führe NASM aus...
[LINK] Verlinke mit Runtime-Bibliothek...
[SUCCESS] Ausführbare Datei: programm
```

---

## Compiler-Interna

### Lexer

**Datei**: `src/lexer.ssl`

**Verantwortlichkeiten**:
- Quellcode tokenisieren
- String-Escapes behandeln
- Schlüsselwörter erkennen
- Literale unterstützen (hex, float, binär)

### Parser

**Datei**: `src/parser.ssl`

**Algorithmus**: Recursive Descent mit Pratt Expression Parsing

**Produktionen**:
```
programm    ::= funktion*
funktion    ::= 'fn' IDENT '(' params ')' '->' typ block
anweisung   ::= let_stmt | if_stmt | while_stmt | return_stmt
ausdruck    ::= primär | binär_op | unär_op
```

### Typ-Prüfer

**Datei**: `src/types.ssl`

**Typ-Regeln**:
```
Γ ⊢ e : τ   (Ausdruck e hat Typ τ im Kontext Γ)

Γ ⊢ n : Int          (Integer-Literal)
Γ ⊢ 3.14 : Float     (Float-Literal)
Γ, x:τ ⊢ x : τ       (Variable)
```

### IR-Generator

**Datei**: `src/ir.ssl`

**SSA-Konstruktion**:
```ssl
fn generiere_ir(ast: AST) -> IR {
    // Konvertiere AST zu SSA-Form
    // φ-Funktionen für Zusammenführungen
    // Basic-Block-Konstruktion
}
```

### Optimizer

**Datei**: `src/optimize.ssl`

**Durchläufe**:
1. **Konstantenfaltung**: `2 + 3` → `5`
2. **Eliminierung toten Codes**: Entferne unerreichbaren Code
3. **Gemeinsame Teilausdrucks-Eliminierung**: Wiederverwendung berechneter Werte

### Code-Generator

**Datei**: `src/codegen.ssl`

**Ziel**: x64 Assembly (NASM-Syntax)

**Register-Allokation**:
- RAX: Rückgabewert, Temporäre
- RBX-R15: Allgemein (Callee-saved)
- RDI, RSI, RDX, RCX, R8, R9: Funktionsargumente

**Stack-Frame**:
```
rbp + 16:  Argument 1
rbp + 8:   Argument 2
rbp + 0:   Gesichertes RBP
rbp - 8:   Lokale Variable 1
rbp - 16:  Lokale Variable 2
...
```

---

## Runtime-Bibliothek

SSL v7.0 enthält eine minimale Runtime-Bibliothek:

**Features**:
- Speicher-Allokation (malloc/free Wrapper)
- String-Operationen
- I/O-Funktionen (print, read)
- Mathematische Funktionen

**Verlinkung**:
```bash
# Automatisch während Kompilierung verlinkt
ssl compile programm.ssl
# Verlinkt mit ssl_runtime.lib
```

---

## Performance-Tipps

### 1. Release-Builds verwenden

```bash
ssl compile --release -O3 programm.ssl
```

### 2. Code profilieren

```bash
# System-Profiler verwenden
perf record ./programm
perf report
```

### 3. Allokationen minimieren

```ssl
// Schlecht: Neue String-Allokation
let ergebnis = string_concat(a, b)

// Besser: Buffer wiederverwenden (wenn API verfügbar)
```

### 4. Geeignete Typen verwenden

```ssl
// Int für Zähler verwenden, nicht Float
let mut zaehler: Int = 0  // Gut
let mut zaehler: Float = 0.0  // Schlecht (langsamer)
```

---

## Fehlersuche

### Häufige Fehler

**1. NASM nicht gefunden**
```
Error: NASM Assembler nicht in PATH gefunden
```
**Lösung**: NASM installieren und zu PATH hinzufügen

**2. Linker-Fehler**
```
Error: undefined reference to `_start`
```
**Lösung**: Sicherstellen, dass Runtime-Bibliothek verlinkt ist

**3. Cross-Compiler nicht gefunden**
```
Error: aarch64-linux-gnu-gcc nicht gefunden
```
**Lösung**: ARM64 Toolchain installieren

---

**Weiter**: [Betriebssystem-Entwicklung →](OS_DEV_DE.md)

**Siehe auch**:
- [Sprachreferenz](SPRACHREFERENZ_DE.md)
- [Erste Schritte](GETTING_STARTED_DE.md)
- [Optimierungs-Leitfaden](OPTIMIERUNG_DE.md)
