# SSL Handbuch

## Einführung

Sonner Studio Language (SSL) ist eine moderne Programmiersprache für die KI-Ära. Dieses Handbuch behandelt die grundlegenden Sprachfunktionen und Syntax.

## Inhaltsverzeichnis

1. [Erste Schritte](#erste-schritte)
2. [Variablen](#variablen)
3. [Typen](#typen)
4. [Funktionen](#funktionen)
5. [Kontrollfluss](#kontrollfluss)
6. [Pattern Matching](#pattern-matching)
7. [Datenstrukturen](#datenstrukturen)
8. [Module](#module)
9. [Fehlerbehandlung](#fehlerbehandlung)
10. [Erweiterte Funktionen](#erweiterte-funktionen)

---

## Erste Schritte

### Hallo Welt

```ssl
fn main() {
    print("Hallo, SSL!")
}
```

### SSL-Code ausführen

```bash
# Datei ausführen
ssl run main.ssl

# Interaktive REPL
ssl run

# Syntax prüfen
ssl check main.ssl
```

---

## Variablen

### Unveränderlich (Standard)

```ssl
let name = "SSL"
let alter = 1
let pi = 3.14159
```

### Veränderlich

```ssl
var zaehler = 0
zaehler = zaehler + 1
```

### Typ-Annotationen

```ssl
let name: String = "SSL"
let zahlen: List<Int> = [1, 2, 3]
```

---

## Typen

### Primitive Typen

| Typ | Beschreibung | Beispiel |
|-----|--------------|----------|
| Int | 64-Bit Ganzzahl | `42` |
| Float | 64-Bit Gleitkomma | `3.14` |
| Bool | Wahrheitswert | `true`, `false` |
| String | UTF-8 Zeichenkette | `"hallo"` |
| Char | Unicode-Zeichen | `'a'` |

### Zusammengesetzte Typen

```ssl
// Listen
let zahlen = [1, 2, 3, 4, 5]

// Maps
let person = {
    name: "Alice",
    alter: 30
}

// Tupel
let punkt = (10, 20)
```

### Eigene Typen

```ssl
// Struct
struct Benutzer {
    name: String
    email: String
    alter: Int
}

// Enum (Summentyp)
type Ergebnis<T, E> = Ok(T) | Err(E)
type Option<T> = Some(T) | None
```

---

## Funktionen

### Einfache Funktionen

```ssl
fn addiere(a: Int, b: Int) -> Int {
    a + b
}

fn begruesse(name: String) {
    print("Hallo, ${name}!")
}
```

### Lambda-Ausdrücke

```ssl
let verdoppeln = |x| x * 2
let addieren = |a, b| a + b

let zahlen = [1, 2, 3, 4, 5]
let verdoppelt = zahlen.map(|x| x * 2)
```

---

## Kontrollfluss

### If/Else

```ssl
if bedingung {
    // ...
} else if andere_bedingung {
    // ...
} else {
    // ...
}

// If als Ausdruck
let max = if a > b { a } else { b }
```

### Schleifen

```ssl
// For-Schleife
for element in liste {
    print(element)
}

// For mit Bereich
for i in 0..10 {
    print(i)
}

// While-Schleife
while bedingung {
    // ...
}
```

---

## Pattern Matching

```ssl
match wert {
    0 => print("null")
    1 | 2 => print("eins oder zwei")
    n if n < 10 => print("klein: ${n}")
    _ => print("anderer")
}

// Destrukturierung
match punkt {
    (0, 0) => print("Ursprung")
    (x, 0) => print("auf X-Achse bei ${x}")
    (0, y) => print("auf Y-Achse bei ${y}")
    (x, y) => print("bei (${x}, ${y})")
}
```

---

## Datenstrukturen

### Structs

```ssl
struct Punkt {
    x: Float
    y: Float
}

impl Punkt {
    fn neu(x: Float, y: Float) -> Punkt {
        Punkt { x, y }
    }
    
    fn abstand(self, anderer: Punkt) -> Float {
        sqrt((self.x - anderer.x)^2 + (self.y - anderer.y)^2)
    }
}
```

### Enums

```ssl
type Farbe = Rot | Gruen | Blau | Rgb(Int, Int, Int)

fn farbname(f: Farbe) -> String {
    match f {
        Rot => "rot"
        Gruen => "grün"
        Blau => "blau"
        Rgb(r, g, b) => "rgb(${r}, ${g}, ${b})"
    }
}
```

---

## Module

### Importieren

```ssl
import math { sqrt, sin, cos }
import http { Request, Response }
import json

// Import mit Alias
import langer_modulname as kurz
```

---

## Fehlerbehandlung

### Result-Typ

```ssl
fn dividiere(a: Float, b: Float) -> Result<Float, String> {
    if b == 0.0 {
        Err("Division durch Null")
    } else {
        Ok(a / b)
    }
}

// Mit ? Operator
fn berechne() -> Result<Float, String> {
    let x = dividiere(10.0, 2.0)?
    let y = dividiere(x, 3.0)?
    Ok(y)
}
```

---

## Erweiterte Funktionen (v4.0)

### Property-Based Testing
```ssl
@property(iterations: 1000)
fn addition_ist_kommutativ(a: Int, b: Int) -> Bool {
    a + b == b + a
}
```

### Reaktive Streams
```ssl
let stream = Stream.from([1, 2, 3])
    .map(|x| x * 2)
    .subscribe(|v| print(v))
```

### Edge Deployment
```ssl
@edge(memory: 128)
fn handler(request: Request) -> Response {
    Response.json({ nachricht: "Hallo!" })
}
```

### Formale Verifikation
```ssl
@requires(n >= 0)
@ensures(result >= 0)
fn fakultaet(n: Int) -> Int {
    if n <= 1 { 1 } else { n * fakultaet(n - 1) }
}
```

### Lineare Typen
```ssl
@linear
struct Datei { handle: DateiHandle }

let datei = Datei.oeffnen("test.txt")
datei.schliessen()  // Pflicht! Kompilierfehler wenn vergessen
```

Weitere Details zu v4.0 Features siehe [V4.0_FEATURES.md](V4.0_FEATURES.md).

---

*SSL v4.0.0 - Dezember 2024*
