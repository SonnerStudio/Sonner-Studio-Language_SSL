# Sonner Studio Language (SSL)

![SSL Logo](assets/Sonner-Studio-Language-Logo.png)

**Un langage de programmation universel et natif pour l'IA du futur**

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)](#tests)

---

## 🌍 Langues

**[English](README.md)** | **[Deutsch](README_DE.md)** | **[Français](README_FR.md)** | **[Español](README_ES.md)** | **[Português](README_PT.md)** | **[日本語](README_JA.md)**

---

## 🌟 Points Forts

SSL est un **langage de programmation expérimental** qui combine des concepts modernes et futuristes :

- ⚡ **Parallel-by-Design** : Support natif des threads et du passage de messages
- ⚛️ **Informatique Quantique** : Simulateur quantique intégré
- 🩹 **Code Auto-Réparateur** : Gestion automatique des erreurs avec intégration de l'IA
- 🤖 **AI-Native** : Compilateur avec optimisation par IA et analyse d'erreurs
- 🔄 **Système de Types Hybride** : Statique + Dynamique + Inférence

---

## 🚀 Démarrage Rapide

### Installation

```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL
cargo build --release
```

### Votre Premier Programme SSL

```ssl
fn main() {
    print("Bonjour, SSL !")
}
```

Exécuter :
```bash
cargo run -- run examples/hello.ssl
```

---

## 📖 Fonctionnalités

### 1. Concurrence ⚡

```ssl
let chan = channel()

spawn {
    send(chan[0], "Bonjour depuis le thread !")
}

print(recv(chan[1]))
```

### 2. Informatique Quantique ⚛️

```ssl
let q = Qubit()
H(q)  // Porte de Hadamard : Superposition
let result = Measure(q)
print(result)  // 0 ou 1 (50/50)
```

### 3. Code Auto-Réparateur 🩹

```ssl
try {
    let result = risky_operation()
} recover (err) {
    print("Erreur capturée :", err)
    // Récupération automatique
}
```

### 4. Fonctions & Récursivité

```ssl
fn fib(n: Int) -> Int {
    if n <= 1 { return n }
    return fib(n-1) + fib(n-2)
}

print(fib(10))  // 55
```

---

## 🛠️ Chaîne d'Outils

### Commandes CLI

```bash
ssl run <file>      # Exécuter le code
ssl build <path>    # Compiler le projet
ssl check <file>    # Vérifier la syntaxe
ssl doctor          # Vérification du système
ssl lsp             # Démarrer le serveur de langue
```

### Démon IA (ssld)

```bash
ssld  # Démarre le démon IA pour l'analyse de code
```

---

## 🤝 Contribution

SSL est un projet expérimental. Les contributions sont les bienvenues !

Veuillez lire [CONTRIBUTING.md](docs/CONTRIBUTING.md) pour les directives.

---

**Construit avec ❤️ et Rust** 🦀
