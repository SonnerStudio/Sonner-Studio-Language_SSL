## 🌍 Langues / Languages

**[English](DOCUMENTATION.md)** | **[Deutsch](DOCUMENTATION_DE.md)** | **[Français](DOCUMENTATION_FR.md)** | **[Español](DOCUMENTATION_ES.md)** | **[Português](DOCUMENTATION_PT.md)** | **[日本語](DOCUMENTATION_JA.md)**

---

# Documentation SSL

## Table des matières

1. [Introduction](#introduction)
2. [Fonctionnalités](#fonctionnalités)
3. [Installation](#installation)
4. [Premiers pas](#premiers-pas)
5. [Syntaxe](#syntaxe)
6. [Fonctions intégrées](#fonctions-intégrées)
7. [Exemples](#exemples)
8. [FAQ](#faq)

---

## Introduction

Sonner Studio Language (SSL) est un langage de programmation expérimental natif de l'IA conçu pour l'avenir de l'informatique. Il combine :

- **Parallèle par conception**: Concurrence native avec threads et canaux
- **Informatique quantique**: Simulateur quantique intégré
- **Code auto-réparant**: Récupération automatique d'erreurs avec assistance IA
- **Système de types moderne**: Typage hybride statique/dynamique avec inférence

---

## Fonctionnalités

### 1. Variables et types

```ssl
let x = 10
mut count = 0
count = count + 1
let name: String = "SSL"
```

### 2. Fonctions

```ssl
fn add(a: Int, b: Int) -> Int {
    return a + b
}
```

### 3. Flux de contrôle

```ssl
if x > 10 {
    print("Plus grand")
} else {
    print("Plus petit")
}

for i in 0..10 {
    print(i)
}
```

### 4. Programmation parallèle

```ssl
let chan = channel()

spawn {
    send(chan[0], 42)
}

let result = recv(chan[1])
print(result)
```

### 5. Informatique quantique

```ssl
let q = Qubit()
H(q)  // Hadamard: Superposition
let result = Measure(q)
print(result)  // 0 ou 1 (50/50)
```

### 6. Gestion des erreurs

```ssl
try {
    let result = operation_risquee()
} recover (err) {
    print("Erreur:", err)
}
```

---

## Installation

```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL
cargo build --release
```

---

## Premiers pas

Créez `hello.ssl`:

```ssl
fn main() {
    print("Bonjour, monde!")
}
```

Exécutez:

```bash
ssl run hello.ssl
```

---

## Syntaxe

### Types de données

- `Int`: Nombres entiers (64-bit)
- `String`: Chaînes de texte
- `Bool`: Booléen (true/false)
- `Qubit`: Bits quantiques

### Opérateurs

**Arithmétique**: `+`, `-`, `*`, `/`
**Comparaison**: `==`, `!=`, `<`, `>`, `<=`, `>=`

---

## Fonctions intégrées

- `print(...args)`: Afficher dans la console
- `channel()`: Créer un canal de communication
- `send(sender, value)`: Envoyer une valeur
- `recv(receiver)`: Recevoir une valeur
- `spawn { ... }`: Démarrer un thread
- `Qubit()`: Créer un qubit
- `H(qubit)`: Hadamard gate
- `Measure(qubit)`: Mesurer un qubit

---

## Exemples

### Pattern Producer-Consumer

```ssl
fn main() {
    let chan = channel()
    
    spawn {
        for i in 0..10 {
            send(chan[0], i)
        }
    }
    
    for i in 0..10 {
        print("Reçu:", recv(chan[1]))
    }
}
```

---

## FAQ

**SSL est-il prêt pour la production?**
Non, SSL est expérimental.

**Quelles plateformes sont supportées?**
Toutes les plateformes Rust (Windows, macOS, Linux).

---

**Construit avec ❤️ et Rust** 🦀
