# SSL Speicherverwaltung (Memory Management)

## Übersicht
SSL verwendet ein hybrides Speichermodell, das die Geschwindigkeit von manueller Speicherverwaltung (wie in C/Rust) mit der Sicherheit eines Garbage Collectors (wie in Python/Java) kombiniert.

## 1. Das Duale Modell

### A. Arena Allocation (Der "Fast Path")
- **Konzept**: Jeder Funktionsaufruf (Stack Frame) erhält eine eigene "Arena" (einen linearen Speicherblock).
- **Verwendung**: Lokale Variablen, temporäre Objekte, kurze Listen.
- **Vorteil**:
    - Allokation ist nur eine Pointer-Inkrementierung (Nanosekunden).
    - Deallokation geschieht *sofort* und *komplett*, wenn die Funktion endet (Arena Reset).
    - Keine Fragmentierung.
- **Rust-Implementierung**: Wir nutzen eine Bibliothek wie `bumpalo` oder eine eigene Implementierung.

### B. Global Heap (Der "Shared Path")
- **Konzept**: Klassischer Heap für Objekte, die den Funktionsaufruf überleben oder sehr groß sind.
- **Verwendung**: Globale Zustände, große Datensätze, Objekte, die zwischen Threads geteilt werden.
- **Verwaltung**: Ein **Generational Garbage Collector (GC)**.
    - **Generation 0 (Nursery)**: Klein, wird häufig gescannt.
    - **Generation 1 (Old Gen)**: Objekte, die mehrere GCs überlebt haben.

## 2. Entscheidung: Stack vs. Heap
Der Compiler (Aurora) entscheidet statisch (zur Compile-Zeit), wo ein Objekt landet:
- **Escape Analysis**: Wenn eine Variable die Funktion nicht verlässt -> **Arena**.
- **Escape**: Wenn sie zurückgegeben oder global gespeichert wird -> **Heap**.

## 3. Sicherheit
- **Bounds Checking**: Jeder Zugriff auf Arrays/Listen wird geprüft.
- **Null Safety**: Es gibt kein `null` (außer in `unsafe` Blöcken). Wir nutzen `Option<T>`.
- **Lifetime Tracking**: Der Compiler verhindert, dass Referenzen auf Arena-Objekte länger leben als die Arena selbst.

## 4. Implementierungsplan (Phase 2)
1.  Integration von `bumpalo` für Arenas.
2.  Implementierung eines einfachen Mark-and-Sweep GC für den Heap.
3.  Später Upgrade auf Generational GC.

---
*Status: Entwurf v0.1*
