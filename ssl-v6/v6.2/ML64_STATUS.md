# ML64 Not Found - Setup Required

## Status

❌ ml64.exe ist nicht im PATH verfügbar

## Nächste Schritte

### Option 1: Visual Studio Build Tools installieren (Empfohlen)

Siehe `TOOLCHAIN_SETUP.md` für Details:

1. Download: https://visualstudio.microsoft.com/downloads/
2. Installer: "Build Tools for Visual Studio 2022"
3. Workload: "Desktop development with C++"
4. Install size: ~2-3 GB

Nach Installation:
```powershell
# Developer Command Prompt öffnen ODER
"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# Dann ml64 wird verfügbar sein
where.exe ml64
```

### Option 2: Alternative Ansatz (Ohne ml64)

Da ml64 derzeit nicht verfügbar ist, können wir:

**A) Mock Implementation** für v6.2 Lexer Development:
- Builtins als Platzhalter implementieren
- Mit Dummy-Werten arbeiten
- Fokus auf Algorithmus-Logik
- Tests mit bekannten Inputs

**B) v5.0 Builtins nutzen** (Hybrid):
- v5.0 hat `string_length()`, `char_at()` etc.
- v6.2 Code ruft v5.0 Builtins auf
- Kompiliert und läuft mit v5.0
- Später durch Runtime ersetzt

## Empfehlung

Für v6.2 Development **jetzt**:
→ **Option 2B: v5.0 Builtins nutzen**

Vorteile:
- ✅ Sofort verfügbar
- ✅ Keine Toolchain-Setup
- ✅ Funktioniert mit v5.0 Compiler
- ✅ Kann getestet werden

Dann später:
→ **Option 1: ml64 installieren** für production builds

---

**Created**: 2025-12-07 13:59
**Status**: Toolchain Check Complete
