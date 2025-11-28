# Sonner Studio Language (SSL)

![SSL Logo](assets/Sonner-Studio-Language-Logo.png)

**Un lenguaje de programación universal y nativo de IA del futuro**

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)](#tests)

---

## 🌍 Idiomas

**[English](README.md)** | **[Deutsch](README_DE.md)** | **[Français](README_FR.md)** | **[Español](README_ES.md)** | **[Português](README_PT.md)** | **[日本語](README_JA.md)**

---

## 🌟 Destacados

SSL es un **lenguaje de programación experimental** que combina conceptos modernos y futuristas:

- ⚡ **Parallel-by-Design**: Soporte nativo para hilos y paso de mensajes
- ⚛️ **Computación Cuántica**: Simulador cuántico integrado
- 🩹 **Código Autocurativo**: Manejo automático de errores con integración de IA
- 🤖 **Nativo de IA**: Compilador con optimización de IA y análisis de errores
- 🔄 **Sistema de Tipos Híbrido**: Estático + Dinámico + Inferencia

---

## 🚀 Inicio Rápido

### Instalación

```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL
cargo build --release
```

### Tu Primer Programa SSL

```ssl
fn main() {
    print("¡Hola, SSL!")
}
```

Ejecutar:
```bash
cargo run -- run examples/hello.ssl
```

---

## 📖 Características

### 1. Concurrencia ⚡

```ssl
let chan = channel()

spawn {
    send(chan[0], "¡Hola desde el hilo!")
}

print(recv(chan[1]))
```

### 2. Computación Cuántica ⚛️

```ssl
let q = Qubit()
H(q)  // Puerta Hadamard: Superposición
let result = Measure(q)
print(result)  // 0 o 1 (50/50)
```

### 3. Código Autocurativo 🩹

```ssl
try {
    let result = risky_operation()
} recover (err) {
    print("Error capturado:", err)
    // Recuperación automática
}
```

### 4. Funciones y Recursividad

```ssl
fn fib(n: Int) -> Int {
    if n <= 1 { return n }
    return fib(n-1) + fib(n-2)
}

print(fib(10))  // 55
```

---

## 🛠️ Cadena de Herramientas

### Comandos CLI

```bash
ssl run <file>      # Ejecutar código
ssl build <path>    # Compilar proyecto
ssl check <file>    # Comprobar sintaxis
ssl doctor          # Comprobación del sistema
ssl lsp             # Iniciar servidor de lenguaje
```

### Demonio de IA (ssld)

```bash
ssld  # Inicia el demonio de IA para análisis de código
```

---

## 🤝 Contribución

SSL es un proyecto experimental. ¡Las contribuciones son bienvenidas!

Por favor lee [CONTRIBUTING.md](docs/CONTRIBUTING.md) para las directrices.

---

**Construido con ❤️ y Rust** 🦀
