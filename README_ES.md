# Sonner Studio Language (SSL)

![SSL Logo](assets/Sonner-Studio-Language-Logo.png)

**Un lenguaje de programación universal nativo de IA**

[![Versión](https://img.shields.io/badge/versión-0.1.0-blue.svg)](https://github.com/yourusername/ssl)
[![Licencia](https://img.shields.io/badge/licencia-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-aprobados-brightgreen.svg)](#tests)

---

## 🌍 Idiomas / Languages

**[English](README.md)** | **[Deutsch](README_DE.md)** | **[Français](README_FR.md)** | **[Español](README_ES.md)** | **[Português](README_PT.md)** | **[日本語](README_JA.md)**

---

## 🌟 Aspectos destacados

SSL es un **lenguaje de programación experimental** que combina conceptos modernos y futuristas:

- ⚡ **Paralelo por diseño**: Soporte nativo para threads y paso de mensajes
- ⚛️ **Computación cuántica**: Simulador cuántico integrado
- 🩹 **Código auto-reparable**: Manejo automático de errores con integración de IA
- 🤖 **Nativo de IA**: Compilador con optimización y análisis de errores mediante IA
- 🔄 **Sistema de tipos híbrido**: Estático + Dinámico + Inferencia

---

## 🚀 Inicio rápido

### Instalación

```bash
git clone https://github.com/yourusername/ssl.git
cd ssl
cargo build --release
```

### Tu primer programa SSL

```ssl
fn main() {
    print("¡Hola, SSL!")
}
```

Ejecutar:
```bash
ssl run examples/hello.ssl
```

---

## 📖 Características

### 1. Paralelismo ⚡

```ssl
let chan = channel()

spawn {
    send(chan[0], "¡Hola desde el thread!")
}

print(recv(chan[1]))
```

### 2. Computación Cuántica ⚛️

```ssl
let q = Qubit()
H(q)  // Hadamard: Superposición
let result = Measure(q)
print(result)  // 0 o 1 (50/50)
```

### 3. Código Auto-Reparable 🩹

```ssl
try {
    let result = operacion_arriesgada()
} recover (err) {
    print("Error capturado:", err)
    // Recuperación automática
}
```

---

## 🛠️ Toolchain

### Comandos CLI

```bash
ssl run <archivo>   # Ejecutar código
ssl build <ruta>    # Compilar proyecto
ssl check <archivo> # Verificar sintaxis
ssl doctor          # Verificación del sistema
ssl lsp             # Iniciar servidor de lenguaje
```

---

## 🧪 Tests

```bash
cargo test
```

**Estado**: Todos los 9 tests unitarios aprobados ✅

---

## 📄 Licencia

Licencia MIT - ver [LICENSE](LICENSE) para detalles.

---

**Construido con ❤️ y Rust** 🦀
