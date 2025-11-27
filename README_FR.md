# Sonner Studio Language (SSL)

**Un langage de programmation universel et natif de l'IA**

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/yourusername/ssl)
[![Licence](https://img.shields.io/badge/licence-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-réussis-brightgreen.svg)](#tests)

---

## 🌍 Langues / Languages

**[English](README.md)** | **[Deutsch](README_DE.md)** | **[Français](README_FR.md)** | **[Español](README_ES.md)** | **[Português](README_PT.md)** | **[日本語](README_JA.md)**

---

## 🌟 Points forts

SSL est un **langage de programmation expérimental** qui combine des concepts modernes et futuristes :

- ⚡ **Parallèle par conception**: Support natif des threads et du passage de messages
- ⚛️ **Informatique quantique**: Simulateur quantique intégré
- 🩹 **Code auto-réparant**: Gestion automatique des erreurs avec intégration IA
- 🤖 **Natif IA**: Compilateur avec optimisation et analyse d'erreurs par IA
- 🔄 **Système de types hybride**: Statique + Dynamique + Inférence

---

## 🚀 Démarrage rapide

### Installation

```bash
git clone https://github.com/yourusername/ssl.git
cd ssl
cargo build --release
```

### Votre premier programme SSL

```ssl
fn main() {
    print("Bonjour, SSL!")
}
```

Exécution:
```bash
ssl run examples/hello.ssl
```

---

## 📖 Fonctionnalités

### 1. Parallélisme ⚡

```ssl
let chan = channel()

spawn {
    send(chan[0], "Bonjour du thread!")
}

print(recv(chan[1]))
```

### 2. Informatique Quantique ⚛️

```ssl
let q = Qubit()
H(q)  // Hadamard: Superposition
let result = Measure(q)
print(result)  // 0 ou 1 (50/50)
```

### 3. Code Auto-Réparant 🩹

```ssl
try {
    let result = operation_risquee()
} recover (err) {
    print("Erreur capturée:", err)
    // Récupération automatique
}
```

---

## 🛠️ Toolchain

### Commandes CLI

```bash
ssl run <fichier>   # Exécuter le code
ssl build <path>    # Compiler le projet
ssl check <fichier> # Vérifier la syntaxe
ssl doctor          # Vérification système
ssl lsp             # Démarrer le serveur de langage
```

---

## 🧪 Tests

```bash
cargo test
```

**Statut**: Tous les 9 tests unitaires réussis ✅

---

## 📄 Licence

Licence MIT - voir [LICENSE](LICENSE) pour les détails.

---

**Construit avec ❤️ et Rust** 🦀
