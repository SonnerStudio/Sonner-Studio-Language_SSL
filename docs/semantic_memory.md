# Semantic Code Memory (SCM) Spezifikation

## Vision
Das "Langzeitgedächtnis" des Compilers. Es ermöglicht der KI, Fragen zum gesamten Projektkontext zu beantworten, nicht nur zur aktuellen Datei.

## 1. Funktionsweise

### A. Indizierung (The "Learning Phase")
-   **Trigger**: Beim Speichern einer Datei oder `ssl build`.
-   **Prozess**:
    1.  **Chunking**: Code wird in logische Blöcke zerlegt (Funktionen, Klassen).
    2.  **Embedding**: Ein Embedding-Modell (z.B. `nomic-embed-text` oder `all-MiniLM-L6-v2`) wandelt den Code in Vektoren um.
    3.  **Storage**: Vektoren werden in einer lokalen Vektor-DB gespeichert (z.B. `LanceDB` oder `Qdrant` im Embedded Mode).

### B. Retrieval (The "Recall Phase")
-   **Trigger**: Wenn die KI eine Frage beantworten oder Code generieren muss.
-   **Prozess**:
    1.  Der Prompt (z.B. "Implementiere eine Methode wie `User.login`") wird ebenfalls eingebettet.
    2.  **Similarity Search**: Die DB findet die relevantesten Code-Stellen im Projekt (z.B. die `User`-Klasse und existierende Auth-Funktionen).
    3.  **Context Injection**: Die gefundenen Snippets werden in den Kontext des LLMs geladen (RAG - Retrieval Augmented Generation).

## 2. Datenstruktur

```json
{
  "id": "hash_of_content",
  "file_path": "src/auth/user.ssl",
  "type": "function",
  "name": "login",
  "content": "fn login(user: String, pass: String) -> Bool { ... }",
  "embedding": [0.12, -0.45, ...],
  "last_updated": 1715000000
}
```

## 3. Privacy & Performance
-   **Lokal**: Die DB liegt im `.ssl/` Ordner des Projekts. Nichts verlässt den Rechner.
-   **Inkrementell**: Nur geänderte Dateien werden neu berechnet.

## 4. Anwendungsfälle
-   **Style-Guide Einhaltung**: "Schreibe Code im Stil dieses Projekts."
-   **Refactoring**: "Ändere die Signatur von `process` überall."
-   **Onboarding**: "Erkläre mir, wie das Routing in diesem Projekt funktioniert."

---
*Status: Entwurf v0.1*
