# Sonner Studio Language (SSL)

![SSL Logo](assets/Sonner-Studio-Language-Logo.png)

**Uma linguagem de programação universal e nativa de IA do futuro**

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)](#tests)

---

## 🌍 Idiomas

**[English](README.md)** | **[Deutsch](README_DE.md)** | **[Français](README_FR.md)** | **[Español](README_ES.md)** | **[Português](README_PT.md)** | **[日本語](README_JA.md)**

---

## 🌟 Destaques

SSL é uma **linguagem de programação experimental** que combina conceitos modernos e futuristas:

- ⚡ **Parallel-by-Design**: Suporte nativo para threads e passagem de mensagens
- ⚛️ **Computação Quântica**: Simulador quântico integrado
- 🩹 **Código Autocurativo**: Tratamento automático de erros com integração de IA
- 🤖 **Nativo de IA**: Compilador com otimização de IA e análise de erros
- 🔄 **Sistema de Tipos Híbrido**: Estático + Dinâmico + Inferência

---

## 🚀 Início Rápido

### Instalação

```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL
cargo build --release
```

### Seu Primeiro Programa SSL

```ssl
fn main() {
    print("Olá, SSL!")
}
```

Executar:
```bash
cargo run -- run examples/hello.ssl
```

---

## 📖 Recursos

### 1. Concorrência ⚡

```ssl
let chan = channel()

spawn {
    send(chan[0], "Olá da thread!")
}

print(recv(chan[1]))
```

### 2. Computação Quântica ⚛️

```ssl
let q = Qubit()
H(q)  // Porta Hadamard: Superposição
let result = Measure(q)
print(result)  // 0 ou 1 (50/50)
```

### 3. Código Autocurativo 🩹

```ssl
try {
    let result = risky_operation()
} recover (err) {
    print("Erro capturado:", err)
    // Recuperação automática
}
```

### 4. Funções e Recursão

```ssl
fn fib(n: Int) -> Int {
    if n <= 1 { return n }
    return fib(n-1) + fib(n-2)
}

print(fib(10))  // 55
```

---

## 🛠️ Ferramentas

### Comandos CLI

```bash
ssl run <file>      # Executar código
ssl build <path>    # Compilar projeto
ssl check <file>    # Verificar sintaxe
ssl doctor          # Verificação do sistema
ssl lsp             # Iniciar servidor de linguagem
```

### Daemon de IA (ssld)

```bash
ssld  # Inicia o daemon de IA para análise de código
```

---

## 🤝 Contribuição

SSL é um projeto experimental. Contribuições são bem-vindas!

Por favor, leia [CONTRIBUTING.md](docs/CONTRIBUTING.md) para diretrizes.

---

**Construído com ❤️ e Rust** 🦀
