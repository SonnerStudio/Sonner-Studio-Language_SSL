# SSL v7.0 Installation

## Quick Installation (Recommended)

SSL v7.0 is distributed as a **binary-only release** to protect proprietary source code.

### Installation via Cargo

```bash
cargo install --git https://github.com/SonnerStudio/Sonner-Studio-Language_SSL
```

This will install the `ssl` command globally.

# Git (falls nicht vorhanden)
winget install Git.Git
```

### Schritt 1: Repository klonen

```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL
```

### Schritt 2: SSL v7 Compiler verwenden

SSL v7 ist ein **self-hosting compiler** - der Compiler ist in SSL selbst geschrieben!

**Dateien im `ssl-v7/src/` Verzeichnis:**
- `main.ssl` - Compiler Entry Point
- `lexer.ssl` - Lexikalische Analyse
- `parser.ssl` - Syntaktische Analyse
- `ir.ssl` - Intermediate Representation
- `optimize.ssl` - Optimizer
- `codegen.ssl` - x64 Code Generator
- `codegen_nasm.ssl` - NASM Assembly Output
- `types.ssl` - Type System
- `error.ssl` - Error Handling

### Schritt 3: Ersten SSL-Code kompilieren

**Einfachstes Beispiel:**
```ssl
fn main() -> Int {
    print("Hello, SSL v7!")
    return 0
}
```

Speichern als `hello.ssl` und kompilieren:

```bash
# Mit dem Kernel-Compiler (empfohlen)
python ssl-v7/kernel_compiler.ssl hello.ssl

# Oder mit dem Haupt-Compiler
python ssl-v7/src/main.ssl hello.ssl
```

### Schritt 4: NASM Assembly generieren

```bash
# Code wird zu NASM Assembly kompiliert
python ssl-v7/src/codegen_nasm.ssl hello.ssl > hello.asm

# Assembly zu Object-File
nasm -f win64 hello.asm -o hello.obj

# Linken (Windows)
link hello.obj /SUBSYSTEM:CONSOLE /ENTRY:main /OUT:hello.exe

# Ausführen
hello.exe
```

### Schnell-Installation (Ein-Befehls-Setup)

```bash
# Klonen und testen
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL
python ssl-v7/test_simple.ssl
```

## 🤖 Ollama Integration

### Installation

1. **Ollama installieren:**
   ```powershell
   winget install Ollama.Ollama
   ```

2. **Modelle laden:**
   ```bash
   ollama pull codellama
   ollama pull mistral
   ```

3. **SSL + Ollama Test:**
   ```bash
   python ssl_ollama_test.py
   ```

### Ollama-Features in SSL v7

**Code-Generierung:**
```python
import requests

def generate_ssl_code(prompt):
    response = requests.post('http://localhost:11434/api/generate',
        json={"model": "codellama", "prompt": prompt, "stream": False})
    return response.json()['response']

# Beispiel
code = generate_ssl_code("Schreibe eine SSL v7 Fibonacci-Funktion")
print(code)
```

**Code-Validierung:**
```python
def validate_code(ssl_code):
    prompt = f"Prüfe diesen SSL-Code auf Fehler:\n{ssl_code}"
    feedback = generate_ssl_code(prompt)
    return feedback
```

### Ollama in SSL verwenden (geplant für v7.1)

```ssl
import Ollama from "ai"

fn main() -> Int {
    let model = Ollama.load("codellama")
    
    // Code generieren
    let code = model.generate("Schreibe eine Fibonacci-Funktion")
    print(code)
    
    // Code validieren
    let validation = model.validate(code)
    if validation.has_errors {
        print("Fehler: " + validation.errors)
    }
    
    return 0
}
```

## 📁 Dateistruktur

```
Sonner-Studio-Language_SSL/
├── ssl-v7/              # SSL v7 Compiler (in SSL geschrieben!)
│   ├── src/
│   │   ├── main.ssl
│   │   ├── lexer.ssl
│   │   ├── parser.ssl
│   │   ├── ir.ssl
│   │   ├── optimize.ssl
│   │   ├── codegen.ssl
│   │   └── codegen_nasm.ssl
│   ├── kernel_compiler.ssl  # ZetaTron-OS Integration
│   ├── test_simple.ssl
│   └── RELEASE_NOTES_v7.0.md
├── docs/
│   ├── GETTING_STARTED.md
│   ├── LANGUAGE_REFERENCE.md
│   ├── COMPILER_GUIDE.md
│   ├── NLP_GUIDE.md
│   └── OLLAMA_INTEGRATION.md
├── examples/
├── README.md
└── ssl_ollama_test.py   # Ollama Integration Test
```

## ✅ Verifikation

```bash
# 1. Repository klonen
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL

# 2. Test ausführen
python ssl-v7/test_simple.ssl

# 3. Ollama testen (falls installiert)
python ssl_ollama_test.py

# Erwartete Ausgabe:
# ✅ SSL v7 Compiler funktioniert
# ✅ NASM Assembly wird generiert
# ✅ Ollama-Integration funktioniert
```

## 🚨 Häufige Probleme

### "Python nicht gefunden"
```powershell
winget install Python.Python.3.11
# Neustart der Shell
```

### "NASM nicht gefunden"
```powershell
winget install NASM.NASM
# Path aktualisieren
```

### "Ollama nicht erreichbar"
```powershell
# Ollama starten
ollama serve
```

## 📖 Weitere Ressourcen

- [Getting Started](docs/GETTING_STARTED.md)
- [NLP Guide](docs/NLP_GUIDE.md)
- [Ollama Integration](docs/OLLAMA_INTEGRATION.md)
- [Compiler Guide](docs/COMPILER_GUIDE.md)

---

**SSL v7.0 ist jetzt vollständig installierbar mit allen Source-Dateien! 🚀**
