# SSL Package Manager (SSPM) Spezifikation

## Vision
Ein Paketmanager, der nicht nur Versionen prüft, sondern **Kompatibilität**.
Schluss mit "Dependency Hell" durch semantische Verträge statt reiner Nummern.

## 1. Das Manifest: `ssl.pkg`

Das Manifest ist selbst gültiger SSL-Code (Data-as-Code), wird aber deklarativ ausgewertet.

```ssl
package {
    name: "my-app",
    version: "1.0.0",
    type: "app", // oder "lib"
    authors: ["Dev One <dev@example.com>"],
    license: "MIT"
}

dependencies {
    // Klassische SemVer
    "std": "1.2.0",
    
    // Semantische Abhängigkeit (Das Killer-Feature)
    // "Ich brauche eine Lib, die das 'Matrix-Solver-Interface' v2 implementiert"
    "matrix-solver": implements("MatrixAPI", "v2"),
    
    // Direkte Git/Pfad Referenzen
    "utils": git("https://github.com/user/utils.git", branch="main")
}

permissions {
    // Capability-Based Security
    // Das Paket darf NUR das, was hier steht.
    allow network("https://api.example.com")
    allow file_read("./data/*")
    // allow system_exec() // Wäre nötig für Subprozesse
}

scripts {
    build: "ssl build src/main.ssl",
    test: "ssl test"
}
```

## 2. Semantische Abhängigkeiten

Statt stur `lib-a v2.3.4` zu fordern, kann ein Paket **Fähigkeiten** (Capabilities/Interfaces) fordern.

-   **Provider-Modell**: Bibliotheken deklarieren `implements("InterfaceName", "Version")`.
-   **Consumer-Modell**: Apps fordern `implements("InterfaceName", "Version")`.
-   **Resolution**: Der Paketmanager wählt automatisch die beste/sicherste/schnellste Bibliothek aus, die das Interface erfüllt (konfigurierbar).

## 3. Sicherheitsmodell (The "Lockdown")

SSPM ist der Wächter.
-   **Installation**: Prüft Signaturen und "Reputation Score" (optional via Community-Datenbank).
-   **Laufzeit**: Generiert ein `security.policy` File für die VM basierend auf dem `permissions` Block.
    -   Wenn eine Lib versucht, `/etc/passwd` zu lesen, aber keine Permission hat -> **Crash mit SecurityException**.

## 4. Registry & Dezentralisierung

-   **Default**: `pkg.sonner.studio` (Offizielle Registry).
-   **Federation**: Firmen können eigene Registries betreiben.
-   **No-Registry**: Git-URLs und lokale Pfade sind First-Class-Citizens.

## 5. Lockfile: `ssl.lock`
Generiert, deterministisch. Speichert exakte Hashes aller Abhängigkeiten (inklusive transitiver).

---
*Status: Entwurf v0.1*
