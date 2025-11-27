# Implementation Plan - Phase 4: Toolchain

## Ziel
Funktionierende Entwicklungsumgebung (`ssl` CLI, `ssld` Daemon, VS Code Extension)

---

## 1. Build-Fehler beheben ❌ BLOCKIERT

### Problem
- `cargo build` schlägt fehl (sowohl `ssl` als auch `ssld`)
- Fehlerdetails in `build.log` (unklar, abgeschnitten)

### Nächste Schritte
1. `build.log` analysieren (vollständige Fehlermeldung)
2. Abhängigkeiten prüfen (`Cargo.toml`)
3. Hot-Fix durchführen

---

## 2. Parser implementieren ⏳

### Status
- Lexer ✅ vorhanden
- AST-Strukturen fehlen

### Aufgaben
- [ ] AST-Datenstrukturen (`src/ast.rs`)
- [ ] Parser-Logik (Recursive Descent oder `pest`/`nom`)
- [ ] Integration in `ssl run`

---

## 3. AI Daemon fertigstellen ⏳

### Aufgaben
- [ ] IPC-Kommunikation (Named Pipes/Unix Sockets)
- [ ] Llama-Modell laden (`candle` Crate)
- [ ] RPC-Methoden:
    - `ping` ✅
    - `explain_error`
    - `expand_natural`

---

## 4. LSP-Server ⏳

### Aufgaben
- [ ] LSP-Protokoll-Handler (`tower-lsp` Crate)
- [ ] Features:
    - Syntax-Highlighting via TextMate ✅
    - Auto-Completion
    - Diagnostics (Fehler anzeigen)
    - Hover (Dokumentation)

---

## 5. VS Code Extension ⏳

### Aufgaben
- [ ] Extension `package.json` ✅
- [ ] LSP-Client konfigurieren
- [ ] Publish zu VS Code Marketplace (später)

---

## Timeline (Schätzung)

| Task | Dauer | Status |
|------|-------|--------|
| Build-Fehler | 1-2h | ❌ |
| Parser | 1 Woche | ⏳ |
| ssld | 3-4 Tage | ⏳ |
| LSP | 1 Woche | ⏳ |

---

## Blockers
- **KRITISCH**: Build-Fehler muss zuerst behoben werden
