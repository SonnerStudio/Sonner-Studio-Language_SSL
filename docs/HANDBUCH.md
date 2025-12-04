# Sonner Studio Language (SSL) v3.1 - Das Offizielle Handbuch

## Inhaltsverzeichnis
1.  [Einführung](#1-einführung)
2.  [Installation & Einrichtung](#2-installation--einrichtung)
3.  [Sprachgrundlagen](#3-sprachgrundlagen)
4.  [Funktionale Programmierung (v3.0)](#4-funktionale-programmierung)
5.  [Der Aurora JIT-Compiler (v3.1)](#5-der-aurora-jit-compiler)
6.  [Standard-Bibliothek](#6-standard-bibliothek)
7.  [Tools & Ökosystem](#7-tools--ökosystem)

---

## 1. Einführung

Willkommen bei **SSL v3.1**, der Programmiersprache für die Zukunft der Softwareentwicklung. SSL verbindet die Sicherheit und Ausdrucksstärke der funktionalen Programmierung mit der rohen Leistung eines nativen JIT-Compilers, eingebettet in ein entwicklerfreundliches Ökosystem mit KI-Tools und Time-Travel-Debugging.

### Kernphilosophie
-   **Hybrides Paradigma:** Mische imperativen und funktionalen Code nahtlos.
-   **Performance First:** Native Maschinencode-Generierung via LLVM.
-   **Developer Experience:** Tools, die deinen Code verstehen (KI, Hot Reload).

---

## 2. Installation & Einrichtung

### Voraussetzungen
-   Rust Toolchain (aktuellste Stable-Version)
-   LLVM 17.0 (für JIT-Support)

### Bauen aus dem Quellcode
```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL
cd Sonner-Studio-Language_SSL
cargo build --release --features llvm
```

### Dein erstes Programm
Erstelle eine Datei namens `hallo.ssl`:
```ssl
print("Hallo, Welt!")
```

Ausführen:
```bash
ssl run hallo.ssl
```

---

## 3. Sprachgrundlagen

### Variablen
Variablen sind in SSL v3.0+ standardmäßig **unveränderlich (immutable)**.
```ssl
let x = 10      // Unveränderlich
// x = 20       // Fehler!

let mut y = 10  // Veränderlich (Mutable)
y = 20          // OK
```

### Datentypen
-   `Int`: 64-Bit Integer (vorzeichenbehaftet)
-   `Float`: 64-Bit Fließkommazahl
-   `String`: UTF-8 Zeichenkette
-   `Bool`: `true` oder `false`
-   `List<T>`: Dynamisches Array
-   `Map<K, V>`: Schlüssel-Wert-Speicher

### Kontrollfluss
```ssl
if x > 10 {
    print("Groß")
} else {
    print("Klein")
}

// Schleifen
for i in 0..10 {
    print(i)
}
```

### Funktionen
```ssl
fn add(a: Int, b: Int) -> Int {
    return a + b
}
```

---

## 4. Funktionale Programmierung

SSL v3.0 führt mächtige funktionale Features ein.

### Der Pipe-Operator (`|>`)
Verkette Funktionsaufrufe zu lesbaren Daten-Pipelines.
```ssl
// Alter Weg
let res = square(double(5))

// SSL Weg
let res = 5 |> double |> square
```

### Auto-Currying
Funktionen können teilweise angewendet werden (Partial Application).
```ssl
fn multiply(a: Int, b: Int) -> Int { return a * b }

let double = multiply(2)  // Gibt eine neue Funktion zurück
print(double(10))         // Gibt 20 aus
```

### Immutability
Datenstrukturen sind standardmäßig unveränderlich. Änderungen geben *neue* Kopien zurück.
```ssl
let list = [1, 2, 3]
let new_list = list.push(4)
// list ist immer noch [1, 2, 3]
// new_list ist [1, 2, 3, 4]
```

---

## 5. Der Aurora JIT-Compiler

SSL v3.1 verfügt über einen hochmodernen LLVM-basierten JIT-Compiler.

### Funktionsweise
1.  **Interpretation:** Code startet sofort im Interpreter.
2.  **Hot-Path-Erkennung:** Die Runtime identifiziert häufig ausgeführte Funktionen.
3.  **Kompilierung:** "Heiße" Funktionen werden im Hintergrund zu nativem Maschinencode kompiliert.
4.  **Optimierung:** LLVM wendet aggressive Optimierungen an (Inlining, Constant Folding).
5.  **Native Ausführung:** Nachfolgende Aufrufe nutzen den ultraschnellen nativen Code.

### Performance
Benchmarks zeigen einen **5-10x Geschwindigkeitsvorteil** bei rechenintensiven Aufgaben im Vergleich zum Interpreter.

---

## 6. Standard-Bibliothek

SSL kommt mit einer "Batteries Included" Standard-Bibliothek.

-   `std::io` - Datei- und Konsolen-I/O
-   `std::net` - HTTP Client/Server
-   `std::json` - JSON Parsing/Serialisierung
-   `std::math` - Fortgeschrittene Mathematik
-   `std::async` - Channels und Futures

---

## 7. Tools & Ökosystem

### Time-Travel Debugger
Schreite in der Zeit *zurück*, um Bugs zu finden.
```bash
ssl run app.ssl --debug
# Nutze @back um zurückzugehen
```

### KI Code Review
Erhalte sofortiges Feedback zu deinem Code.
```bash
ssl run app.ssl --ai-review
```

### Hot Reload
Ändere Code, während die App läuft.
```bash
ssl run app.ssl --watch
```

---

*© 2024 Sonner Studio. Lizenziert unter MIT/Apache 2.0.*
