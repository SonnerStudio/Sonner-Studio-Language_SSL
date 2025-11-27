# Aurora Compiler Architektur

## Übersicht
Der Aurora Compiler ist der zentrale Transformationsmotor von SSL. Er ist als hybride Pipeline konzipiert, die klassische Compilertheorie mit moderner KI-gesteuerter Optimierung verbindet.

## Pipeline-Stufen

### 1. Frontend (Rust)
- **Lexer**: Tokenisiert den Quellcode. Verarbeitet gemischte Modi (Formal + Natürlich).
- **Parser**: Generiert einen Abstrakten Syntaxbaum (AST) basierend auf der EBNF-Grammatik.
- **Semantic Analyzer**:
    - Führt statische Typprüfung durch.
    - Löst Symbole und Gültigkeitsbereiche (Scopes) auf.
    - Validiert Ownership-Regeln (falls aktiviert).

### 2. AI Semantic Core (Python/Rust Bridge)
- **Input**: AST + Natürlichsprachliche Blöcke.
- **Prozess**:
    - **Intent Extraction**: Analysiert `natural { ... }` Blöcke und `@ai` Dekoratoren.
    - **Expansion**: Ersetzt natürlichsprachliche Absichten durch formale SSL-AST-Teilbäume.
    - **Fehlererklärung**: Wenn das Frontend scheitert, analysiert der Core den Fehlerkontext für menschenlesbare Lösungen.

### 3. Optimizer (Aurora AI)
- **Klassische Pässe**: Dead Code Elimination, Loop Unrolling, Constant Folding (LLVM-basiert).
- **KI-Pässe**:
    - **Mustererkennung**: Identifiziert High-Level-Algorithmen (z. B. "Bubble Sort") und ersetzt sie durch optimierte Standardbibliotheksaufrufe.
    - **Profile-Guided Optimization (PGO)**: Nutzt historische Laufzeitdaten zur Optimierung häufig genutzter Pfade.

### 4. Backend
- **Intermediate Representation (SSL-IR)**: Eine High-Level-IR, die die semantische Absicht bewahrt.
- **Lowering**:
    - **Zu LLVM IR**: Für native CPU/GPU-Ausführung.
    - **Zu WASM**: Für Web-Ziele.
    - **Zu SSL Bytecode**: Für den SSLVM-Interpreter.

## Technologie-Stack
- **Sprache**: Rust (Core), Python (KI-Prototyping, später Portierung zu Rust/TorchScript).
- **IR**: MLIR / LLVM.
- **KI-Modelle**: Lokale SLMs (Small Language Models), z. B. quantisiertes Llama/Mistral.
