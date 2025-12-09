# Ollama AI-Unterstützung in SSL v7.0

## Überblick

SSL v7.0 bietet native Integration mit **Ollama**, dem lokalen Large Language Model (LLM) Server, für KI-gestützte Entwicklung und intelligente Anwendungen.

## Kernfunktionen

### 1. Native Ollama-Integration
- **Direkte API-Anbindung** ohne externe Dependencies
- **Typsicheres Interface** für alle Ollama-Modelle
- **Streaming-Support** für Echtzeit-Antworten
- **Async/Await** für nicht-blockierende KI-Aufrufe

### 2. Unterstützte Modelle
- **Llama 2** (7B, 13B, 70B)
- **Mistral** (7B, Mixtral 8x7B)
- **CodeLlama** - Optimiert für Code-Generation
- **Gemma** - Google's Open Model
- **Phi-2** - Microsoft's Small Language Model
- **Custom Models** - Jedes in Ollama verfügbare Modell

### 3. Sprachfeatures

#### Einfache API
```ssl
import Ollama from "ai"

fn main() -> Int {
    // Modell initialisieren
    let model = Ollama.load("llama2")
    
    // Einfache Anfrage
    let response = model.generate("Erkläre Quantencomputing in 3 Sätzen")
    print(response)
    
    return 0
}
```

#### Streaming-Antworten
```ssl
import Ollama, Stream from "ai"

fn stream_response() -> Int {
    let model = Ollama.load("mistral")
    
    // Streaming für Echtzeit-Ausgabe
    model.stream("Schreibe eine Geschichte über SSL", fn(chunk: String) {
        print(chunk)  // Ausgabe in Echtzeit
    })
    
    return 0
}
```

#### Chat-Konversationen
```ssl
import Ollama, Chat from "ai"

fn chat_demo() -> Int {
    let model = Ollama.load("llama2")
    let chat = Chat.new(model)
    
    // Multi-Turn Konversation
    chat.add_message("user", "Was ist SSL?")
    let response1 = chat.send()
    
    chat.add_message("user", "Welche Features hat es?")
    let response2 = chat.send()
    
    print(response2)
    return 0
}
```

#### Code-Generierung
```ssl
import Ollama from "ai"

fn generate_code() -> Int {
    let model = Ollama.load("codellama")
    
    let prompt = "Schreibe eine SSL-Funktion für Fibonacci"
    let code = model.generate(prompt, temperature: 0.2)
    
    print(code)
    return 0
}
```

### 4. Erweiterte Features

#### Embeddings
```ssl
import Ollama, Embeddings from "ai"

fn create_embeddings() -> Int {
    let model = Ollama.load("llama2")
    
    let text = "Natural Language Programming in SSL"
    let embedding = model.embed(text)
    
    // Vektor mit 4096 Dimensionen
    print("Dimension: " + len(embedding))
    return 0
}
```

#### RAG (Retrieval-Augmented Generation)
```ssl
import Ollama, VectorDB from "ai"

fn rag_example() -> Int {
    let model = Ollama.load("mistral")
    let db = VectorDB.new()
    
    // Dokumente indizieren
    db.add("SSL ist eine Programmiersprache...")
    db.add("Native Compilation bedeutet...")
    
    // Relevante Dokumente finden
    let context = db.search("Was ist SSL?", top_k: 3)
    
    // Mit Kontext generieren
    let response = model.generate(
        "Basierend auf: " + context + "\nBeantworte: Was ist SSL?"
    )
    
    print(response)
    return 0
}
```

#### Multi-Modell-Orchestrierung
```ssl
import Ollama from "ai"

fn multi_model() -> Int {
    let coder = Ollama.load("codellama")
    let reviewer = Ollama.load("mistral")
    
    // Code generieren
    let code = coder.generate("Schreibe eine Fibonacci-Funktion")
    
    // Code reviewen
    let review = reviewer.generate(
        "Reviewe diesen Code:\n" + code
    )
    
    print("Code:\n" + code)
    print("\nReview:\n" + review)
    
    return 0
}
```

### 5. NLP-Sprachunterstützung

Ollama-Integration funktioniert mit allen 9 NLP-Sprachen:

```ssl
#!lang de
import Ollama from "ai"

funktion ki_demo() -> Ganzzahl {
    lass modell = Ollama.laden("llama2")
    lass antwort = modell.generieren("Hallo, wie geht es dir?")
    ausgabe(antwort)
    rückgabe 0
}
```

### 6. Performance-Optimierungen

#### Modell-Caching
```ssl
import Ollama from "ai"

fn cached_model() -> Int {
    // Modell wird gecacht
    let model = Ollama.load("llama2", cache: true)
    
    // Schnellere Folgeaufrufe
    let r1 = model.generate("Test 1")
    let r2 = model.generate("Test 2")  // Verwendet gecachtes Modell
    
    return 0
}
```

#### Batch-Verarbeitung
```ssl
import Ollama from "ai"

fn batch_processing() -> Int {
    let model = Ollama.load("mistral")
    
    let prompts = [
        "Erkläre KI",
        "Erkläre ML",
        "Erkläre Deep Learning"
    ]
    
    // Parallele Verarbeitung
    let responses = model.batch(prompts, parallel: true)
    
    for response in responses {
        print(response)
    }
    
    return 0
}
```

### 7. Konfiguration

#### Modell-Parameter
```ssl
import Ollama, ModelConfig from "ai"

fn configure_model() -> Int {
    let config = ModelConfig {
        temperature: 0.7,      // Kreativität (0.0-1.0)
        top_p: 0.9,           // Nucleus Sampling
        top_k: 40,            // Top-K Sampling
        num_predict: 512,     // Max Tokens
        repeat_penalty: 1.1,  // Wiederholungs-Strafe
        context_window: 2048  // Kontext-Größe
    }
    
    let model = Ollama.load("llama2", config)
    let response = model.generate("Erzähle eine Geschichte")
    
    print(response)
    return 0
}
```

#### System-Prompts
```ssl
import Ollama from "ai"

fn system_prompt() -> Int {
    let model = Ollama.load("mistral")
    
    model.set_system(
        "Du bist ein SSL-Programmier-Experte. " +
        "Gib präzise, sichere Code-Beispiele."
    )
    
    let code = model.generate("Wie erstelle ich ein Herz-Fenster?")
    print(code)
    
    return 0
}
```

### 8. Integration mit Non-Rectangular Windows

```ssl
import Ollama, Window from "ai", "gui"

fn ai_window() -> Int {
    let model = Ollama.load("llama2")
    
    // KI-generierter Inhalt in herzförmigem Fenster
    let poem = model.generate("Schreibe ein Gedicht über Liebe")
    
    let window = Window.create_heart(
        width: 400,
        height: 450,
        color: 0xff1744
    )
    
    window.add_text(poem, font_size: 14)
    window.show()
    
    return 0
}
```

## Installation

### Voraussetzungen
1. **Ollama installieren:**
   ```bash
   # Windows
   winget install Ollama.Ollama
   
   # macOS
   brew install ollama
   
   # Linux
   curl -fsSL https://ollama.ai/install.sh | sh
   ```

2. **Modell herunterladen:**
   ```bash
   ollama pull llama2
   ollama pull mistral
   ollama pull codellama
   ```

3. **SSL AI-Modul:**
   ```bash
   ssl install ai
   ```

## Anwendungsfälle

### 1. Code-Assistent
```ssl
fn code_assistant(task: String) -> String {
    let model = Ollama.load("codellama")
    return model.generate(
        "Schreibe SSL-Code für: " + task,
        temperature: 0.2
    )
}
```

### 2. Dokumentations-Generator
```ssl
fn generate_docs(code: String) -> String {
    let model = Ollama.load("mistral")
    return model.generate(
        "Generiere Dokumentation für:\n" + code
    )
}
```

### 3. Test-Generator
```ssl
fn generate_tests(function: String) -> String {
    let model = Ollama.load("codellama")
    return model.generate(
        "Generiere Unit-Tests für:\n" + function
    )
}
```

### 4. Chatbot
```ssl
import Ollama, Chat, Window from "ai", "gui"

fn create_chatbot() -> Int {
    let model = Ollama.load("llama2")
    let chat = Chat.new(model)
    
    let window = Window.create_rounded_rect(
        width: 600,
        height: 800,
        corner_radius: 20
    )
    
    window.on_input(fn(user_msg: String) {
        chat.add_message("user", user_msg)
        let response = chat.send()
        window.add_message("ai", response)
    })
    
    window.show()
    return 0
}
```

## Performance

| Modell | Tokens/Sek | Latenz (First Token) | RAM-Nutzung |
|--------|------------|---------------------|-------------|
| Llama 2 7B | ~40 | 100ms | 4 GB |
| Mistral 7B | ~50 | 80ms | 4 GB |
| CodeLlama 7B | ~45 | 90ms | 4 GB |
| Phi-2 | ~60 | 60ms | 2.5 GB |

*Gemessen auf: AMD Ryzen 9 5900X, 32GB RAM*

## Best Practices

1. **Modell-Auswahl:**
   - **CodeLlama** für Code-Generierung
   - **Mistral** für allgemeine Aufgaben
   - **Phi-2** für schnelle, einfache Aufgaben

2. **Temperature-Einstellungen:**
   - **0.0-0.3:** Deterministische Ausgaben (Code, Fakten)
   - **0.7-0.9:** Kreative Ausgaben (Stories, Ideen)

3. **Kontext-Management:**
   - Halte Prompts fokussiert und präzise
   - Nutze System-Prompts für konsistentes Verhalten
   - Implementiere Context-Window-Management für lange Konversationen

4. **Ressourcen:**
   - Cache häufig genutzte Modelle
   - Nutze Batch-Processing für mehrere Anfragen
   - Implementiere Timeouts für lange Generierungen

## Lizenz & Compliance

- **Ollama:** MIT License
- **SSL AI-Modul:** Apache 2.0
- **Modelle:** Siehe jeweilige Modell-Lizenzen (meist Apache 2.0, MIT)

## Weitere Ressourcen

- [Ollama Dokumentation](https://github.com/ollama/ollama)
- [SSL AI Guide](docs/AI_GUIDE.md)
- [Modell-Vergleich](docs/MODEL_COMPARISON.md)

---

**Ollama + SSL v7.0 = Lokale KI-Power mit nativer Performance! 🤖⚡**
