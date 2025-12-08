# Natural Language Programming (NLP) in SSL v7.0

## Overview

SSL v7.0 introduces **Natural Language Programming (NLP)**, allowing developers to write code using natural language constructs in addition to traditional syntax. This revolutionary feature makes programming accessible to a global audience.

## Table of Contents

1. [Introduction](#introduction)
2. [Supported Languages](#supported-languages)
3. [Basic Usage](#basic-usage)
4. [Language Examples](#language-examples)
5. [NLP Syntax Reference](#nlp-syntax-reference)
6. [Advanced Features](#advanced-features)
7. [Best Practices](#best-practices)

---

## Introduction

### What is NLP in SSL?

Natural Language Programming in SSL allows you to:
- Write function names in your native language
- Use natural language keywords
- Mix traditional SSL syntax with NLP constructs
- Collaborate internationally with multilingual code

### Key Benefits

- **Accessibility**: Lower barriers for non-English speakers
- **Expressiveness**: More intuitive code for domain experts
- **Collaboration**: International teams can code in their preferred language
- **Education**: Easier learning curve for beginners

---

## Supported Languages

SSL v7.0 NLP supports **9+ languages** with full keyword and identifier support:

### 🌍 European Languages

| Language | Code | Keywords | Example |
|----------|------|----------|---------|
| **English** | `en` | ✅ Full | `function`, `if`, `while` |
| **German** | `de` | ✅ Full | `funktion`, `wenn`, `solange` |
| **French** | `fr` | ✅ Full | `fonction`, `si`, `pendant` |
| **Spanish** | `es` | ✅ Full | `función`, `si`, `mientras` |
| **Portuguese** | `pt` | ✅ Full | `função`, `se`, `enquanto` |

### 🌏 Middle Eastern & Asian Languages

| Language | Code | Keywords | Example |
|----------|------|----------|---------|
| **Hebrew** | `he` | ✅ Full | `פונקציה`, `אם`, `כל עוד` |
| **Japanese** | `ja` | ✅ Full | `関数`, `もし`, `間` |
| **Hindi** | `hi` | ✅ Full | `फ़ंक्शन`, `यदि`, `जबकि` |
| **Bengali** | `bn` | ✅ Full | `ফাংশন`, `যদি`, `যখন` |

### Additional Language Support

SSL v7.0 also supports **regional dialects** and variations:
- **German**: Standard German, Austrian German, Swiss German
- **English**: US English, UK English, Australian English
- **Spanish**: Castilian, Latin American variants
- **Portuguese**: European Portuguese, Brazilian Portuguese

**Total**: 9 primary languages + 15+ dialects = **24+ language variants**

---

## Basic Usage

### Setting Language

#### In Code (Pragma)

```ssl
#!lang de

funktion hauptprogramm() -> Ganzzahl {
    ausgabe("Hallo, Welt!")
    rückgabe 0
}
```

#### Command Line

```bash
ssl compile --lang de programm.ssl
```

#### Project Configuration

```toml
# ssl.toml
[project]
language = "de"
```

### Mixed Language Code

You can mix languages in the same project:

```ssl
// English module
fn calculate(x: Int) -> Int {
    return x * 2
}

// German module (imported)
#!lang de
fn berechne_summe(a: Ganzzahl, b: Ganzzahl) -> Ganzzahl {
    rückgabe a + b
}
```

---

## Language Examples

### English (Default)

```ssl
fn main() -> Int {
    let x = 42
    if x > 10 {
        print("Greater than 10")
    } else {
        print("Less or equal")
    }
    return 0
}
```

### German (Deutsch)

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

### French (Français)

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

### Spanish (Español)

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

### Japanese (日本語)

```ssl
#!lang ja

関数 メイン() -> 整数 {
    変数 x = 42
    もし x > 10 {
        出力("10より大きい")
    } それ以外 {
        出力("10以下")
    }
    戻り値 0
}
```

### Hindi (हिन्दी)

```ssl
#!lang hi

फ़ंक्शन मुख्य() -> पूर्णांक {
    चर x = 42
    यदि x > 10 {
        प्रिंट("10 से अधिक")
    } अन्यथा {
        प्रिंट("10 से कम या बराबर")
    }
    वापसी 0
}
```

---

## NLP Syntax Reference

### Keywords by Language

#### Control Flow

| English | German | French | Spanish | Japanese | Hindi |
|---------|--------|--------|---------|----------|-------|
| `if` | `wenn` | `si` | `si` | `もし` | `यदि` |
| `else` | `sonst` | `sinon` | `sino` | `それ以外` | `अन्यथा` |
| `while` | `solange` | `pendant` | `mientras` | `間` | `जबकि` |
| `return` | `rückgabe` | `retour` | `retorno` | `戻り値` | `वापसी` |

#### Declarations

| English | German | French | Spanish | Japanese | Hindi |
|---------|--------|--------|---------|----------|-------|
| `fn` | `funktion` | `fonction` | `función` | `関数` | `फ़ंक्शन` |
| `let` | `lass` | `soit` | `sea` | `変数` | `चर` |
| `mut` | `veränderlich` | `mutable` | `mutable` | `可変` | `परिवर्तनीय` |

#### Types

| English | German | French | Spanish | Japanese | Hindi |
|---------|--------|--------|---------|----------|-------|
| `Int` | `Ganzzahl` | `Entier` | `Entero` | `整数` | `पूर्णांक` |
| `Float` | `Fließkomma` | `Flottant` | `Flotante` | `浮動小数点` | `फ्लोट` |
| `String` | `Zeichenkette` | `Chaîne` | `Cadena` | `文字列` | `स्ट्रिंग` |
| `Bool` | `Boolean` | `Booléen` | `Booleano` | `ブール` | `बूलियन` |

#### Built-in Functions

| English | German | French | Spanish | Japanese | Hindi |
|---------|--------|--------|---------|----------|-------|
| `print` | `ausgabe` | `afficher` | `imprimir` | `出力` | `प्रिंट` |
| `read_line` | `zeile_lesen` | `lire_ligne` | `leer_línea` | `行を読む` | `पंक्ति_पढ़ें` |

---

## Advanced Features

### Natural Language Function Names

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

### Natural Language Comments

```ssl
#!lang de

// Dies ist ein Kommentar auf Deutsch
/* 
   Mehrzeiliger Kommentar
   in deutscher Sprache
*/

funktion hauptprogramm() -> Ganzzahl {
    ausgabe("Hallo!")  // Ausgabe des Texts
    rückgabe 0
}
```

### Mixed Language API

```ssl
// English API
fn calculate_total(items: Int) -> Int {
    return items * 10
}

// German usage
#!lang de
funktion zeige_gesamt() {
    lass gesamt = calculate_total(5)  // Mix: German + English API
    ausgabe(ganzzahl_zu_text(gesamt))
}
```

---

## Best Practices

### 1. Consistency Within Modules

```ssl
// ✅ Good: Consistent language per module
#!lang de
funktion berechne_summe(a: Ganzzahl, b: Ganzzahl) -> Ganzzahl {
    rückgabe a + b
}

// ❌ Bad: Mixed keywords in same function
#!lang de
function summe(a: Ganzzahl, b: Ganzzahl) -> Int {  // Mixed!
    return a + b
}
```

### 2. Document Language Choice

```ssl
// Module: Mathematik-Bibliothek
// Sprache: Deutsch
// Autor: Team Deutschland

#!lang de

funktion quadrat(x: Ganzzahl) -> Ganzzahl {
    rückgabe x * x
}
```

### 3. Use English for Libraries

For maximum compatibility, consider English for shared libraries:

```ssl
// Public library - English for wide adoption
fn factorial(n: Int) -> Int {
    if n <= 1 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}
```

### 4. Team Conventions

Establish language standards:

```toml
# team-standards.toml
[coding]
primary_language = "en"        # English for core
documentation_language = "de"  # German for docs
comments_language = "de"       # German for comments
```

---

## IDE Support

SSL v7.0 NLP is supported in:

- **VS Code** - SSL Language Extension
- **IntelliJ IDEA** - SSL Plugin
- **Vim/Neovim** - ssl.vim
- **Emacs** - ssl-mode

### Autocomplete

NLP-aware autocomplete works in all supported languages:

```sql
// Type "wenn" → suggests full if-else structure
wenn | {
    |
} sonst {
    |
}
```

---

## Language Configuration

### Auto-Detection

SSL can auto-detect language from file:

```ssl
// auto-detected as German
funktion hauptprogramm() -> Ganzzahl {
    ausgabe("Auto-erkannt!")
    rückgabe 0
}
```

### Explicit Configuration

```bash
# Compile with specific language
ssl compile --lang de programm.ssl

# Check language
ssl check --lang ja スクリプト.ssl
```

---

## Migration Guide

### From English to Other Languages

**Original (English)**:
```ssl
fn main() -> Int {
    let x = 42
    if x > 10 {
        print("Hello")
    }
    return 0
}
```

**Translated (German)**:
```ssl
#!lang de
funktion hauptprogramm() -> Ganzzahl {
    lass x = 42
    wenn x > 10 {
        ausgabe("Hallo")
    }
    rückgabe 0
}
```

### Automated Translation

```bash
# SSL includes a translation tool
ssl translate --from en --to de program.ssl > program_de.ssl
```

---

## NLP Compiler Support

### Language Detection

```bash
ssl compile program.ssl
# Auto-detects language from #!lang pragma or keywords
```

### Explicit Language

```bash
ssl compile --lang de german_program.ssl
ssl compile --lang ja 日本語プログラム.ssl
```

### Multi-Language Projects

```toml
# ssl.toml
[project]
name = "multi_lang_app"
default_language = "en"

[[modules]]
path = "src/core.ssl"
language = "en"

[[modules]]
path = "src/ui_de.ssl"
language = "de"

[[modules]]
path = "src/ui_ja.ssl"
language = "ja"
```

---

## Community

### Language Support Requests

Request new languages:
- **GitHub**: [Language Request Template](https://github.com/SonnerStudio/SSL/issues/new?template=language_request)
- **Forum**: [Language Discussion](https://discuss.sonnerstudio.com/c/nlp)

### Contributing Translations

See [CONTRIBUTING.md](../CONTRIBUTING.md) for:
- Adding new language keywords
- Improving translations
- Documenting idioms

---

**Next**: [Advanced NLP Features](NLP_ADVANCED.md)

**See Also**:
- [Language Reference](LANGUAGE_REFERENCE.md)
- [Internationalization](I18N.md)
- [Community Guidelines](../CODE_OF_CONDUCT.md)
