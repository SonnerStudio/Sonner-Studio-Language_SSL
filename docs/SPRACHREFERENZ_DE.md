# SSL v7.0 Sprachreferenz

Vollständige Referenz für die Sonner Studio Language v7.0 Native Compilation Edition.

## Inhaltsverzeichnis

1. [Lexikalische Struktur](#lexikalische-struktur)
2. [Typen](#typen)
3. [Variablen](#variablen)
4. [Funktionen](#funktionen)
5. [Kontrollfluss](#kontrollfluss)
6. [Operatoren](#operatoren)
7. [Standardbibliothek](#standardbibliothek)
8. [Speichermodell](#speichermodell)

---

## Lexikalische Struktur

### Kommentare

```ssl
// Einzeiliger Kommentar

/* 
   Mehrzeiliger Kommentar
   Kann mehrere Zeilen umfassen
*/
```

### Schlüsselwörter

Reservierte Wörter in SSL v7.0:

```
fn      let     mut     if      else    while   
return  Int     Float   String  Bool    true    
false   struct  enum    impl    trait   mod
use     pub     priv    const   static  extern
```

### Bezeichner

Gültige Bezeichner-Formate:
- Beginnen mit Buchstabe oder Underscore: `a-z`, `A-Z`, `_`
- Fortgesetzt mit Buchstaben, Ziffern, Underscores: `a-z`, `A-Z`, `0-9`, `_`

```ssl
let gueltiger_name = 42
let _privateVar = true
let MeinTyp = 100
```

**Ungültig:**
```ssl
let 123ungueltig = 0  // Kann nicht mit Ziffer beginnen
let meine-var = 0      // Bindestrich nicht erlaubt
```

### Literale

#### Ganzzahl-Literale

```ssl
let dezimal = 42
let hex = 0x2A
let binaer = 0b101010
let oktal = 0o52
```

#### Fließkomma-Literale

```ssl
let pi = 3.14159
let wissenschaftlich = 1.5e10
let negativer_exp = 2.5e-3
```

#### String-Literale

```ssl
let einfach = "Hallo, Welt!"
let escaped = "Zeile 1\nZeile 2\tTabulator"
let anfuehrung = "Sie sagte \"Hallo\""
let backslash = "Pfad: C:\\Users\\Name"
```

**Escape-Sequenzen:**
- `\n` - Neue Zeile
- `\t` - Tabulator
- `\\` - Backslash
- `\"` - Doppelte Anführungszeichen
- `\r` - Wagenrücklauf

#### Boolean-Literale

```ssl
let flag_wahr = true
let flag_falsch = false
```

---

## Typen

### Primitive Typen

| Typ | Größe | Beschreibung | Bereich |
|-----|-------|--------------|---------|
| `Int` | 64-bit | Signed Integer | -2^63 bis 2^63-1 |
| `Float` | 64-bit | IEEE 754 Double | ±1.7E±308 |
| `Bool` | 1-bit | Boolean | true, false |
| `String` | Variabel | UTF-8 String | Heap-allokiert |

### Typ-Annotationen

```ssl
let x: Int = 42
let y: Float = 3.14
let name: String = "SSL"
let flag: Bool = true
```

### Typ-Inferenz

SSL v7.0 kann Typen ableiten:

```ssl
let x = 42          // Als Int abgeleitet
let y = 3.14        // Als Float abgeleitet
let text = "Hallo"  // Als String abgeleitet
```

---

## Variablen

### Unveränderliche Variablen

Standard-Variablen sind unveränderlich:

```ssl
let x = 42
x = 50  // FEHLER: Kann unveränderlicher Variable nicht zuweisen
```

### Veränderliche Variablen

Verwenden Sie das `mut` Schlüsselwort:

```ssl
let mut zaehler = 0
zaehler = zaehler + 1  // OK
zaehler = 10           // OK
```

### Konstanten

Compile-Zeit-Konstanten:

```ssl
const PI: Float = 3.14159
const MAX_GROESSE: Int = 1024
```

---

## Funktionen

### Funktionsdeklaration

```ssl
fn funktions_name(param1: Typ1, param2: Typ2) -> Rueckgabetyp {
    // Funktionskörper
    return wert
}
```

**Beispiel:**
```ssl
fn addiere(a: Int, b: Int) -> Int {
    return a + b
}
```

### Funktionsaufrufe

```ssl
let ergebnis = addiere(5, 3)
print(int_to_string(ergebnis))
```

### Rückgabewerte

```ssl
fn hole_wert() -> Int {
    return 42
}

fn keine_rueckgabe() {
    print("Kein Rückgabewert")
    // Implizite Rückgabe
}
```

---

## Kontrollfluss

### If-Else Anweisungen

```ssl
if bedingung {
    // Ausführen, wenn wahr
} else {
    // Ausführen, wenn falsch
}
```

**Beispiel:**
```ssl
let x = 10

if x > 5 {
    print("Größer als 5")
} else {
    print("Kleiner oder gleich 5")
}
```

### While-Schleifen

```ssl
while bedingung {
    // Schleifenkörper
}
```

**Beispiel:**
```ssl
let mut i = 0
while i < 10 {
    print(int_to_string(i))
    i = i + 1
}
```

---

## Operatoren

### Arithmetische Operatoren

| Operator | Beschreibung | Beispiel |
|----------|--------------|----------|
| `+` | Addition | `5 + 3` = 8 |
| `-` | Subtraktion | `5 - 3` = 2 |
| `*` | Multiplikation | `5 * 3` = 15 |
| `/` | Division | `6 / 3` = 2 |
| `%` | Modulo | `7 % 3` = 1 |

### Vergleichsoperatoren

| Operator | Beschreibung | Beispiel |
|----------|--------------|----------|
| `==` | Gleich | `5 == 5` = true |
| `!=` | Ungleich | `5 != 3` = true |
| `>` | Größer als | `5 > 3` = true |
| `<` | Kleiner als | `3 < 5` = true |
| `>=` | Größer oder gleich | `5 >= 5` = true |
| `<=` | Kleiner oder gleich | `3 <= 5` = true |

### Logische Operatoren

| Operator | Beschreibung | Beispiel |
|----------|--------------|----------|
| `&&` | Logisches UND | `true && false` = false |
| `\|\|` | Logisches ODER | `true \|\| false` = true |
| `!` | Logisches NICHT | `!true` = false |

---

## Standardbibliothek

### Eingebaute Funktionen

#### I/O-Funktionen

```ssl
print(text: String)              // Ausgabe auf stdout
println(text: String)            // Ausgabe mit Zeilenumbruch
read_line() -> String            // Einlesen von stdin
```

#### String-Funktionen

```ssl
string_length(s: String) -> Int
string_concat(s1: String, s2: String) -> String
substring(s: String, start: Int, len: Int) -> String
```

#### Konversionsfunktionen

```ssl
int_to_string(n: Int) -> String
float_to_string(f: Float) -> String
string_to_int(s: String) -> Int
string_to_float(s: String) -> Float
```

#### Mathematische Funktionen

```ssl
abs(n: Int) -> Int
pow(base: Float, exp: Float) -> Float
sqrt(n: Float) -> Float
floor(n: Float) -> Int
ceil(n: Float) -> Int
```

---

## Speichermodell

### Stack-Allokation

Lokale Variablen werden auf dem Stack allokiert:

```ssl
fn beispiel() -> Int {
    let x = 42      // Stack-allokiert
    let y = 3.14    // Stack-allokiert
    return x
    // x und y automatisch freigegeben
}
```

### String-Allokation

Strings werden auf dem Heap allokiert mit automatischer Verwaltung:

```ssl
fn beispiel() -> String {
    let text = "Hallo, Welt!"  // Heap-allokiert
    return text
    // Speicher automatisch verwaltet
}
```

---

## Versionsunterschiede

### SSL v7.0 vs v5.0

| Feature | v5.0 | v7.0 |
|---------|------|------|
| Kompilierung | Bytecode | Nativ x64 |
| Pattern Matching | ✅ | ⚠️ Geplant v7.1 |
| Generics | ✅ | ⚠️ Geplant v7.2 |
| Algebraische Effekte | ✅ | ⚠️ Geplant v7.3 |
| Multi-Architektur | ❌ | ✅ |
| Performance | 9x Python | 25x Python |

---

**Weiter**: [Compiler-Handbuch →](COMPILER_HANDBUCH_DE.md)

**Siehe auch**:
- [Erste Schritte](GETTING_STARTED_DE.md)
- [Beispiele](../examples/)
- [Standardbibliothek API](STDLIB_DE.md)
