# SSL Toolchain Spezifikation

## 1. Die `ssl` CLI
Das Schweizer Taschenmesser für SSL-Entwickler.

### Befehlsreferenz
-   `ssl new <name>`: Erstellt neues Projekt (mit `ssl.pkg`, `.gitignore`).
-   `ssl run [file]`: Führt Datei oder Projekt aus (JIT/Interpreter).
-   `ssl build`: Kompiliert zu Binary oder Bytecode.
-   `ssl check`: Nur Syntax/Typ-Check (schnell).
-   `ssl fmt`: Formatiert Code (Opinionated, wie `go fmt`).
-   `ssl install`: Installiert Abhängigkeiten aus `ssl.pkg`.
-   `ssl doctor`: Prüft Umgebung (Rust, C++ Build Tools, CUDA/Metal).
-   `ssl ai <query>`: Ad-hoc KI-Anfrage ("Erkläre mir diesen Fehler").

## 2. Der `ssld` (Daemon)
Der unsichtbare Helfer.
-   **Start**: Wird automatisch von `ssl` oder der IDE gestartet.
-   **Aufgabe**:
    -   Hält KI-Modelle im RAM (vermeidet Ladezeit).
    -   Indiziert Code für das Semantic Memory.
    -   Dient als Backend für den Language Server.
-   **Kommunikation**: Named Pipes (Windows) oder Unix Domain Sockets (Linux/Mac).

## 3. IDE Integration (LSP)
Wir implementieren das **Language Server Protocol (LSP)**.
-   **Server**: `ssl-lsp` (Teil des Daemons oder Standalone).
-   **Features**:
    -   Auto-Completion (Syntaktisch + KI-gestützt).
    -   Go to Definition.
    -   Hover Documentation.
    -   **Code Lens**: "AI Explain" Button über komplexen Funktionen.

## 4. VS Code Extension
-   Syntax Highlighting (`.tmLanguage`).
-   Verbindet sich mit `ssl-lsp`.
-   Chat-Sidebar für "Natural Coding".

---
*Status: Entwurf v0.1*
