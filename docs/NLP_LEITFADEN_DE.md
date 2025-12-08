# Natural Language Programming (NLP) in SSL v7.0

## Übersicht

SSL v7.0 führt **Natural Language Programming (NLP)** ein, das Entwicklern ermöglicht, Code in natürlicher Sprache zusätzlich zur traditionellen Syntax zu schreiben. Dieses revolutionäre Feature macht Programmierung für ein globales Publikum zugänglich.

## Inhaltsverzeichnis

1. [Einführung](#einführung)
2. [Unterstützte Sprachen](#unterstützte-sprachen)
3. [Grundlegende Verwendung](#grundlegende-verwendung)
4. [Sprach-Beispiele](#sprach-beispiele)
5. [NLP-Syntax-Referenz](#nlp-syntax-referenz)
6. [Erweiterte Features](#erweiterte-features)
7. [Best Practices](#best-practices)

---

## Einführung

### Was ist NLP in SSL?

Natural Language Programming in SSL ermöglicht Ihnen:
- Funktionsnamen in Ihrer Muttersprache zu schreiben
- Natürlichsprachige Schlüsselwörter zu verwenden
- Traditionelle SSL-Syntax mit NLP-Konstrukten zu mischen
- International mit mehrsprachigem Code zu kollaborieren

### Hauptvorteile

- **Zugänglichkeit**: Niedrigere Barrieren für Nicht-Englischsprachige
- **Ausdrucksstärke**: Intuitiverer Code für Domänen-Experten
- **Zusammenarbeit**: Internationale Teams können in ihrer bevorzugten Sprache programmieren
- **Bildung**: Einfachere Lernkurve für Anfänger

---

## Unterstützte Sprachen

SSL v7.0 NLP unterstützt **9+ Sprachen** mit vollständiger Schlüsselwort- und Bezeichner-Unterstützung:

### 🌍 Europäische Sprachen

| Sprache | Code | Schlüsselwörter | Beispiel |
|---------|------|-----------------|----------|
| **Englisch** | `en` | ✅ Vollständig | `function`, `if`, `while` |
| **Deutsch** | `de` | ✅ Vollständig | `funktion`, `wenn`, `solange` |
| **Französisch** | `fr` | ✅ Vollständig | `fonction`, `si`, `pendant` |
| **Spanisch** | `es` | ✅ Vollständig | `función`, `si`, `mientras` |
| **Portugiesisch** | `pt` | ✅ Vollständig | `função`, `se`, `enquanto` |

### 🌏 Naher Osten & Asiatische Sprachen

| Sprache | Code | Schlüsselwörter | Beispiel |
|---------|------|-----------------|----------|
| **Hebräisch** | `he` | ✅ Vollständig | `פונקציה`, `אם`, `כל עוד` |
| **Japanisch** | `ja` | ✅ Vollständig | `関数`, `もし`, `間` |
| **Hindi** | `hi` | ✅ Vollständig | `फ़ंक्शन`, `यदि`, `जबकि` |
| **Bengali** | `bn` | ✅ Vollständig | `ফাংশন`, `যদি`, `যখন` |

### Zusätzliche Sprachunterstützung

SSL v7.0 unterstützt auch **regionale Dialekte** und Varianten:
- **Deutsch**: Standarddeutsch, Österreichisches Deutsch, Schweizer Deutsch
- **Englisch**: US-Englisch, UK-Englisch, Australisches Englisch
- **Spanisch**: Kastilisch, Lateinamerikanische Varianten
- **Portugiesisch**: Europäisches Portugiesisch, Brasilianisches Portugiesisch

**Gesamt**: 9 Hauptsprachen + 15+ Dialekte = **24+ Sprachvarianten**

---

## Grundlegende Verwendung

### Sprache festlegen

#### Im Code (Pragma)

```ssl
#!lang de

funktion hauptprogramm() -> Ganzzahl {
    ausgabe("Hallo, Welt!")
    rückgabe 0
}
```

#### Kommandozeile

```bash
ssl compile --lang de programm.ssl
```

#### Projekt-Konfiguration

```toml
# ssl.toml
[project]
language = "de"
```

### Mehrsprachiger Code

Sie können Sprachen im selben Projekt mischen:

```ssl
// Englisches Modul
fn calculate(x: Int) -> Int {
    return x * 2
}

// Deutsches Modul (importiert)
#!lang de
fn berechne_summe(a: Ganzzahl, b: Ganzzahl) -> Ganzzahl {
    rückgabe a + b
}
```

---

## Sprach-Beispiele

### Deutsch

```ssl
#!lang de

funktion hauptprogramm() -> Ganzzahl {
    lass x = 42
    wenn x > 10 {
        ausgabe("Größer als 10")
    } sonst {
        ausgabe("Kleiner oder gleich")
    }
    rückgabe 0
}
```

### Französisch

```ssl
#!lang fr

fonction principal() -> Entier {
    soit x = 42
    si x > 10 {
        afficher("Plus grand que 10")
    } sinon {
        afficher("Inférieur ou égal")
    }
    retour 0
}
```

### Spanisch

```ssl
#!lang es

función principal() -> Entero {
    sea x = 42
    si x > 10 {
        imprimir("Mayor que 10")
    } sino {
        imprimir("Menor o igual")
    }
    retorno 0
}
```

---

## NLP-Syntax-Referenz

### Schlüsselwörter nach Sprache

#### Kontrollfluss

| Englisch | Deutsch | Französisch | Spanisch | Japanisch | Hindi |
|----------|---------|-------------|----------|-----------|-------|
| `if` | `wenn` | `si` | `si` | `もし` | `यदि` |
| `else` | `sonst` | `sinon` | `sino` | `それ以外` | `अन्यथा` |
| `while` | `solange` | `pendant` | `mientras` | `間` | `जबकि` |
| `return` | `rückgabe` | `retour` | `retorno` | `戻り値` | `वापसी` |

#### Deklarationen

| Englisch | Deutsch | Französisch | Spanisch | Japanisch | Hindi |
|----------|---------|-------------|----------|-----------|-------|
| `fn` | `funktion` | `fonction` | `función` | `関数` | `फ़ंक्शन` |
| `let` | `lass` | `soit` | `sea` | `変数` | `चर` |
| `mut` | `veränderlich` | `mutable` | `mutable` | `可変` | `परिवर्तनीय` |

#### Typen

| Englisch | Deutsch | Französisch | Spanisch | Japanisch | Hindi |
|----------|---------|-------------|----------|-----------|-------|
| `Int` | `Ganzzahl` | `Entier` | `Entero` | `整数` | `पूर्णांक` |
| `Float` | `Fließkomma` | `Flottant` | `Flotante` | `浮動小数点` | `फ्लोट` |
| `String` | `Zeichenkette` | `Chaîne` | `Cadena` | `文字列` | `स्ट्रिंग` |
| `Bool` | `Boolean` | `Booléen` | `Booleano` | `ブール` | `बूलियन` |

---

## Erweiterte Features

### Natürlichsprachige Funktionsnamen

```ssl
#!lang de

funktion berechne_fibonacci(n: Ganzzahl) -> Ganzzahl {
    wenn n <= 1 {
        rückgabe n
    } sonst {
        rückgabe berechne_fibonacci(n - 1) + berechne_fibonacci(n - 2)
    }
}
```

### Gemischte Sprach-API

```ssl
// Englische API
fn calculate_total(items: Int) -> Int {
    return items * 10
}

// Deutsche Verwendung
#!lang de
funktion zeige_gesamt() {
    lass gesamt = calculate_total(5)  // Mix: Deutsch + Englische API
    ausgabe(ganzzahl_zu_text(gesamt))
}
```

---

## Best Practices

### 1. Konsistenz pro Modul

```ssl
// ✅ Gut: Konsistente Sprache pro Modul
#!lang de
funktion berechne_summe(a: Ganzzahl, b: Ganzzahl) -> Ganzzahl {
    rückgabe a + b
}

// ❌ Schlecht: Gemischte Schlüsselwörter in derselben Funktion
#!lang de
function summe(a: Ganzzahl, b: Ganzzahl) -> Int {  // Gemischt!
    return a + b
}
```

### 2. Sprachwahl dokumentieren

```ssl
// Modul: Mathematik-Bibliothek
// Sprache: Deutsch
// Autor: Team Deutschland

#!lang de

funktion quadrat(x: Ganzzahl) -> Ganzzahl {
    rückgabe x * x
}
```

---

## IDE-Unterstützung

SSL v7.0 NLP wird unterstützt in:

- **VS Code** - SSL Language Extension
- **IntelliJ IDEA** - SSL Plugin
- **Vim/Neovim** - ssl.vim
- **Emacs** - ssl-mode

---

## Community

### Sprachunterstützungs-Anfragen

Neue Sprachen anfordern:
- **GitHub**: [Language Request Template](https://github.com/SonnerStudio/SSL/issues/new?template=language_request)
- **Forum**: [Language Discussion](https://discuss.sonnerstudio.com/c/nlp)

---

**Weiter**: [Erweiterte NLP-Features](NLP_ADVANCED_DE.md)

**Siehe auch**:
- [Sprachreferenz](SPRACHREFERENZ_DE.md)
- [Internationalisierung](I18N_DE.md)
- [Community-Richtlinien](../CODE_OF_CONDUCT.md)
