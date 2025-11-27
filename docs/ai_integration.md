# SSL AI Integration Spezifikation

## Vision
Der Compiler ist nicht mehr nur ein Übersetzer, sondern ein intelligenter Assistent. Er versteht nicht nur Syntax, sondern Absicht.

## 1. Architektur: Der "Brain-Bus"

Der Aurora Compiler (Rust) kommuniziert über eine interne Schnittstelle (den "Brain-Bus") mit dem KI-Modul.

```mermaid
graph TD
    A[Aurora Compiler Core] -- JSON-RPC / SHM --> B[AI Service Daemon]
    B -- Inference --> C[Local SLM (Llama-3 Quantized)]
    B -- Optional API --> D[Cloud LLM (GPT-4/Claude)]
```

### Warum ein Daemon?
-   Das Laden des Modells (GBs an RAM) dauert Sekunden. Der Compiler muss in Millisekunden starten.
-   Der Daemon läuft im Hintergrund (wie `docker d` oder `LSP Server`) und hält das Modell im Speicher.

## 2. Kern-Features

### A. Semantische Fehleranalyse ("Human-Semantic Error System")
Statt `Error: Unexpected token ';'`, sagt SSL:
> "Es sieht so aus, als hättest du ein Semikolon vergessen, aber in SSL nutzen wir Zeilenumbrüche. Meintest du...?"

-   **Trigger**: Bei Syntax/Typ-Fehlern.
-   **Input**: Fehlercode + Umgebungscode + Fehlermeldung.
-   **Output**: Natürliche Erklärung + Fix-Vorschlag.

### B. Natural Language Expansion (`natural { ... }`)
-   **Trigger**: Parser trifft auf `natural`-Block.
-   **Input**: Text im Block + Sichtbare Variablen/Funktionen (Kontext).
-   **Output**: SSL-Code, der anstelle des Blocks kompiliert wird.
-   **Caching**: Das Ergebnis wird gehasht und gecacht. Nur bei Änderung des Prompts wird neu generiert.

### C. AI-Optimierung (`@ai("optimize")`)
-   **Trigger**: `@ai`-Decorator an Funktion.
-   **Prozess**:
    1.  Compiler generiert IR.
    2.  AI analysiert IR und sucht Muster (z.B. "Das ist eine naive Matrix-Multiplikation").
    3.  AI schlägt Ersatz durch `std::math::Matrix.mul()` vor.

## 3. Modell-Strategie

### "Local First"
-   **Standard**: Ein quantisiertes 7B/8B Modell (z.B. Mistral oder Llama-3), das lokal auf CPU/GPU läuft.
-   **Framework**: `candle` (Rust-native ML) oder `llama.cpp` Bindings.
-   **Privacy**: Kein Code verlässt den Rechner, es sei denn, der User aktiviert explizit "Cloud Boost".

## 4. Semantic Code Memory (Vorschau)
Der AI-Service führt eine Vektor-Datenbank (z.B. `qdrant` oder `sqlite-vss`) mit Embeddings des gesamten Projekts.
-   Ermöglicht Fragen wie: "Wo haben wir die User-Authentifizierung implementiert?"
-   Hilft der AI, den Stil des Projekts zu lernen.

---
*Status: Entwurf v0.1*
