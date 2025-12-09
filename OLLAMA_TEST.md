# SSL v7 + Ollama Integration Test

Dieser Test zeigt die Integration von SSL v7 mit Ollama für KI-gestützte Code-Generierung und -Validierung.

## Installation Status

**Hinweis:** SSL v7.0 ist als Binary-Only-Release konzipiert. Der vollständige Compiler ist nicht als Quellcode verfügbar.

Für die Nutzung:
1. Verwenden Sie die vorkompilierten Binaries
2. Oder nutzen Sie den SSL v5/v6 Interpreter mit Ollama-Integration

## Ollama-Integration Beispiele

### 1. Code-Generierung mit Ollama

```python
# SSL v7 Ollama Integration (Python Prototype)
import requests
import json

def ollama_generate(prompt, model="codellama"):
    """Generiere Code mit Ollama"""
    response = requests.post('http://localhost:11434/api/generate', 
        json={
            "model": model,
            "prompt": prompt,
            "stream": False
        })
    return response.json()['response']

# Beispiel: SSL-Code generieren
prompt = """
Schreibe eine SSL v7.0 Funktion, die:
- Fibonacci-Zahlen berechnet
- Typsicher ist (Int -> Int)
- Optimiert für native Compilation
"""

code = ollama_generate(prompt)
print("Generierter Code:")
print(code)
```

### 2. Code-Validierung

```python
def validate_ssl_code(code, model="codellama"):
    """Validiere SSL-Code mit Ollama"""
    prompt = f"""
Analysiere folgenden SSL v7.0 Code auf Fehler:

{code}

Prüfe:
- Syntax-Korrektheit
- Typ-Sicherheit
- Performance-Probleme
- Best Practices

Gib Feedback in Deutsch.
"""
    
    feedback = ollama_generate(prompt, model)
    return feedback

# Test
test_code = """
fn fibonacci(n: Int) -> Int {
    if n <= 1 { return n }
    return fibonacci(n-1) + fibonacci(n-2)
}
"""

feedback = validate_ssl_code(test_code)
print("\nCode-Feedback:")
print(feedback)
```

### 3. NLP-Code-Generierung

```python
def generate_nlp_code(task, language="de", model="mistral"):
    """Generiere SSL-Code in verschiedenen Sprachen"""
    prompt = f"""
Schreibe SSL v7.0 Code für folgende Aufgabe: {task}

Nutze die NLP-Sprache: {language}
Beispiel für Deutsch: funktion, wenn, solange, rückgabe

Gib nur den Code aus, keine Erklärungen.
"""
    
    code = ollama_generate(prompt, model)
    return code

# Beispiel: Deutschen SSL-Code generieren
german_code = generate_nlp_code(
    "Berechne die Summe von 1 bis n", 
    language="de"
)
print("\nDeutscher SSL-Code:")
print(german_code)
```

### 4. Non-Rectangular Window Code-Generator

```python
def generate_window_code(shape, model="codellama"):
    """Generiere Code für nicht-rechteckige Fenster"""
    prompt = f"""
Generiere SSL v7.0 Code für ein {shape}-förmiges Fenster.

Nutze:
- Window.create_{shape}() aus dem "gui" Modul
- Setze sinnvolle Parameter (Größe, Farbe, etc.)
- Füge Text-Label hinzu
- Zeige das Fenster an

Nur Code, keine Erklärungen.
"""
    
    code = ollama_generate(prompt, model)
    return code

# Beispiel: Herz-Fenster generieren
heart_code = generate_window_code("heart")
print("\nHerz-Fenster Code:")
print(heart_code)
```

## Ollama-Modelle für SSL

### Empfohlen:
1. **codellama** - Spezialisiert auf Code-Generierung
2. **mistral** - Gut für allgemeine Aufgaben
3. **phi-2** - Schnell für einfache Aufgaben

### Installation:
```bash
ollama pull codellama
ollama pull mistral
ollama pull phi-2
```

## Live-Test

```bash
# 1. Ollama starten (bereits läuft laut User)
# ollama serve

# 2. Modell testen
ollama run codellama "Schreibe eine SSL v7 Hello World Funktion"

# 3. Python-Integration testen
python ssl_ollama_test.py
```

## Erwartete Ausgabe

### Code-Generierung:
```ssl
fn fibonacci(n: Int) -> Int {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}
```

### Validierung:
```
Code-Feedback:
✓ Syntax korrekt
✓ Typ-Signatur korrekt (Int -> Int)
⚠ Performance: Rekursion ohne Memoization - O(2^n) Komplexität
💡 Empfehlung: Iterative Implementierung oder Memoization nutzen
```

### NLP-Code (Deutsch):
```ssl
#!lang de
funktion summe_bis_n(n: Ganzzahl) -> Ganzzahl {
    lass ergebnis = 0
    lass i = 1
    solange i <= n {
        ergebnis = ergebnis + i
        i = i + 1
    }
    rückgabe ergebnis
}
```

### Window-Code (Herz):
```ssl
import Window from "gui"

fn create_heart_window() -> Int {
    let window = Window.create_heart(
        width: 300,
        height: 320,
        color: 0xff1744,
        glow: true
    )
    
    window.add_label("Made with ❤️ by Ollama + SSL", x: 80, y: 150)
    window.show()
    return 0
}
```

## Integration in SSL v7

Die Ollama-Integration wird in zukünftigen SSL-Versionen nativ unterstützt:

```ssl
// Geplante native Syntax (SSL v7.1+)
import Ollama from "ai"

fn main() -> Int {
    let model = Ollama.load("codellama")
    let code = model.generate("Schreibe eine Fibonacci-Funktion")
    
    // Code validieren
    let validation = model.validate(code)
    if validation.has_errors {
        print("Fehler gefunden: " + validation.errors)
    } else {
        print("Code ist korrekt!")
        print(code)
    }
    
    return 0
}
```

## Nächste Schritte

1. ✅ Ollama läuft
2. ⏳ SSL v7 Binary-Installation
3. ⏳ Python-Prototyp für Ollama-Integration testen
4. 🔜 Native SSL-Integration in v7.1

---

**Status:** Ollama ready, SSL v7 Binary-Installation empfohlen für volle Funktionalität.
