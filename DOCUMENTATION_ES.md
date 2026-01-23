## 🌍 Idiomas / Languages

**[English](DOCUMENTATION.md)** | **[Deutsch](DOCUMENTATION_DE.md)** | **[Français](DOCUMENTATION_FR.md)** | **[Español](DOCUMENTATION_ES.md)** | **[Português](DOCUMENTATION_PT.md)** | **[日本語](DOCUMENTATION_JA.md)**

---

# Documentación SSL

## Índice

1. [Introducción](#introducción)
2. [Características](#características)
3. [Instalación](#instalación)
4. [Primeros pasos](#primeros-pasos)
5. [Sintaxis](#sintaxis)
6. [Funciones integradas](#funciones-integradas)
7. [Ejemplos](#ejemplos)
8. [FAQ](#faq)

---

## Introducción

Sonner Studio Language (SSL) es un lenguaje de programación experimental nativo de IA diseñado para el futuro de la informática. Combina:

- **Paralelo por diseño**: Concurrencia nativa con threads y canales
- **Computación cuántica**: Simulador cuántico integrado
- **Código auto-reparable**: Recuperación automática de errores con asistencia IA
- **Sistema de tipos moderno**: Tipado híbrido estático/dinámico con inferencia

---

## Características

### 1. Variables y tipos

```ssl
let x = 10
mut count = 0
count = count + 1
let name: String = "SSL"
```

### 2. Funciones

```ssl
fn add(a: Int, b: Int) -> Int {
    return a + b
}
```

### 3. Flujo de control

```ssl
if x > 10 {
    print("Mayor")
} else {
    print("Menor")
}

for i in 0..10 {
    print(i)
}
```

### 4. Programación paralela

```ssl
let chan = channel()

spawn {
    send(chan[0], 42)
}

let result = recv(chan[1])
print(result)
```

### 5. Computación cuántica

```ssl
let q = Qubit()
H(q)  // Hadamard: Superposición
let result = Measure(q)
print(result)  // 0 o 1 (50/50)
```

### 6. Manejo de errores

```ssl
try {
    let result = operacion_arriesgada()
} recover (err) {
    print("Error:", err)
}
```

---

## Instalación

```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL
cargo build --release
```

---

## Primeros pasos

Crea `hello.ssl`:

```ssl
fn main() {
    print("¡Hola, mundo!")
}
```

Ejecuta:

```bash
ssl run hello.ssl
```

---

## Sintaxis

### Tipos de datos

- `Int`: Números enteros (64-bit)
- `String`: Cadenas de texto
- `Bool`: Booleano (true/false)
- `Qubit`: Bits cuánticos

### Operadores

**Aritmética**: `+`, `-`, `*`, `/`
**Comparación**: `==`, `!=`, `<`, `>`, `<=`, `>=`

---

## Funciones integradas

- `print(...args)`: Mostrar en consola
- `channel()`: Crear canal de comunicación
- `send(sender, value)`: Enviar valor
- `recv(receiver)`: Recibir valor
- `spawn { ... }`: Iniciar thread
- `Qubit()`: Crear qubit
- `H(qubit)`: Hadamard gate
- `Measure(qubit)`: Medir qubit

---

## Ejemplos

### Patrón Productor-Consumidor

```ssl
fn main() {
    let chan = channel()
    
    spawn {
        for i in 0..10 {
            send(chan[0], i)
        }
    }
    
    for i in 0..10 {
        print("Recibido:", recv(chan[1]))
    }
}
```

---

## FAQ

**¿SSL está listo para producción?**
No, SSL es experimental.

**¿Qué plataformas son compatibles?**
Todas las plataformas Rust (Windows, macOS, Linux).

---

**Construido con ❤️ y Rust** 🦀
