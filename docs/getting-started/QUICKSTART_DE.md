# SSL Schnellstart

**Ihr 5-Minuten-Guide zur Quantenprogrammierung**

---

## Installation

### Voraussetzungen
- Rust Toolchain (https://rust-lang.org)
- Git

### Klonen & Bauen
```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL
cd Sonner-Studio-Language_SSL
cargo build --release
```

---

## Ihr erstes SSL-Programm

`hello.ssl` erstellen:
```ssl
fn main() {
    print("Hallo, SSL!")
}
```

Ausführen:
```bash
cargo run --bin ssl -- run hello.ssl
```

---

## Beispiel 1: Quanten-Zufallszahl
```ssl
fn main() {
    let q = Qubit()
    H(q)  // Hadamard: Superposition
    let result = Measure(q)
    print("Quanten-Bit:", result)  // 0 oder 1 (50/50)
}
```

**Was passiert:**
1. Qubit im |0⟩ Zustand erstellen
2. Hadamard-Gate anwenden (Superposition)
3. Messen (kollabiert zu 0 oder 1)

---

## Beispiel 2: Parallele Programmierung
```ssl
fn main() {
    let chan = channel()
    
    spawn {
        send(chan[0], "Hallo vom Thread!")
    }
    
    let message = recv(chan[1])
    print(message)
}
```

**Kernkonzepte:**
- `spawn`: Parallelen Thread erstellen
- `channel()`: Kommunikationskanal
- `send/recv`: Nachrichtenaustausch

---

## Beispiel 3: Web API & JSON
```ssl
fn main() {
    try {
        // JSON von API abrufen
        let response = http_get("https://api.github.com/users/github")
        let data = json_parse(response)
        
        // In Datei speichern
        fs_write("github.json", json_stringify(data))
        print("Daten gespeichert!")
        
    } recover (err) {
        print("Fehler:", err)
    }
}
```

**Verwendete Features:**
- HTTP Client (`http_get`)
- JSON Parsing (`json_parse`)
- Datei I/O (`fs_write`)
- Fehlerbehandlung (`try/recover`)

---

## Beispiel 4: Maps & Daten
```ssl
fn main() {
    let config = {
        "name": "MeineApp",
        "version": "1.0.0",
        "features": ["quantum", "parallel", "distributed"]
    }
    
    print("App:", config)
    print("JSON:", json_stringify(config))
}
```

---

## Was als Nächstes?

### Beispiele erkunden
```bash
cd examples/
ls *.ssl
```

### Dokumentation lesen
- [Sprachführer](../DOCUMENTATION_DE.md)
- [Standardbibliothek](../stdlib.md)
- [Beispiele](../EXAMPLES.md)

### Community beitreten
- GitHub Discussions
- Issues einreichen
- Beitragen!

---

**Sie sind bereit, mit SSL zu bauen!** 🚀⚛️
