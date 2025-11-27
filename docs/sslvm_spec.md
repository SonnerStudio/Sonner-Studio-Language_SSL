# Sonner Studio Virtual Machine (SSLVM) Spezifikation

## Vision
Eine universelle, sichere und intelligente Laufzeitumgebung, entwickelt um SSL-Code überall auszuführen (Cloud, Edge, Desktop).

## Kernkomponenten

### 1. Execution Engine
- **Dualer Modus**:
    - **Interpreter**: Schneller Start, ideal für Scripting und Entwicklung.
    - **JIT (Just-In-Time)**: Kompiliert häufige Codepfade zu nativem Maschinencode mittels LLVM ORC.

### 2. Speichermodell
- **Hybrides Management**:
    - **Arena Allocation**: Für kurzlebige Objekte (Request-Scope, Frame-Scope). Extrem schnell, null Fragmentierung.
    - **Generational GC**: Für langlebige Heap-Objekte.
- **Sicherheit**:
    - Bounds Checking standardmäßig aktiviert.
    - Optionaler "Unsafe Mode" für rohen Zeigerzugriff (markiert mit `unsafe`).

### 3. Scheduler
- **Green Threads**: M:N Threading-Modell (ähnlich wie Go/Erlang).
- **Async/Await**: First-Class-Support für asynchrone I/O.
- **Parallelität**: Automatische Verteilung von Aufgaben über verfügbare Kerne.

### 4. Security Sandbox
- **Capability-Based Security**: Code kann nicht auf Dateisystem oder Netzwerk zugreifen, es sei denn explizit via `ssl.pkg` erlaubt.
- **Isolation**: Jedes Modul läuft in seinem eigenen logischen Namensraum.

### 5. Bytecode Format (.sslbc)
- Kompakte binäre Repräsentation der SSL-IR.
- Enthält Metadaten für den KI-Optimizer (Intent-Hinweise).

## Interoperabilität (FFI)
- **Zero-Cost C-Interop**: Direkte Kompatibilität der Aufrufkonventionen.
- **Python Bridge**: Eingebetteter Python-Interpreter für Legacy-Bibliotheksunterstützung.
