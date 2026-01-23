## 🌍 Idiomas / Languages

**[English](DOCUMENTATION.md)** | **[Deutsch](DOCUMENTATION_DE.md)** | **[Français](DOCUMENTATION_FR.md)** | **[Español](DOCUMENTATION_ES.md)** | **[Português](DOCUMENTATION_PT.md)** | **[日本語](DOCUMENTATION_JA.md)**

---

# Documentação SSL

## Índice

1. [Introdução](#introdução)
2. [Características](#características)
3. [Instalação](#instalação)
4. [Primeiros passos](#primeiros-passos)
5. [Sintaxe](#sintaxe)
6. [Funções integradas](#funções-integradas)
7. [Exemplos](#exemplos)
8. [FAQ](#faq)

---

## Introdução

Sonner Studio Language (SSL) é uma linguagem de programação experimental nativa de IA projetada para o futuro da computação. Combina:

- **Paralelo por design**: Concorrência nativa com threads e canais
- **Computação quântica**: Simulador quântico integrado
- **Código auto-reparável**: Recuperação automática de erros com assistência IA
- **Sistema de tipos moderno**: Tipagem híbrida estática/dinâmica com inferência

---

## Características

### 1. Variáveis e tipos

```ssl
let x = 10
mut count = 0
count = count + 1
let name: String = "SSL"
```

### 2. Funções

```ssl
fn add(a: Int, b: Int) -> Int {
    return a + b
}
```

### 3. Fluxo de controle

```ssl
if x > 10 {
    print("Maior")
} else {
    print("Menor")
}

for i in 0..10 {
    print(i)
}
```

### 4. Programação paralela

```ssl
let chan = channel()

spawn {
    send(chan[0], 42)
}

let result = recv(chan[1])
print(result)
```

### 5. Computação quântica

```ssl
let q = Qubit()
H(q)  // Hadamard: Superposição
let result = Measure(q)
print(result)  // 0 ou 1 (50/50)
```

### 6. Tratamento de erros

```ssl
try {
    let result = operacao_arriscada()
} recover (err) {
    print("Erro:", err)
}
```

---

## Instalação

```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL
cargo build --release
```

---

## Primeiros passos

Crie `hello.ssl`:

```ssl
fn main() {
    print("Olá, mundo!")
}
```

Execute:

```bash
ssl run hello.ssl
```

---

## Sintaxe

### Tipos de dados

- `Int`: Números inteiros (64-bit)
- `String`: Cadeias de texto
- `Bool`: Booleano (true/false)
- `Qubit`: Bits quânticos

### Operadores

**Aritmética**: `+`, `-`, `*`, `/`
**Comparação**: `==`, `!=`, `<`, `>`, `<=`, `>=`

---

## Funções integradas

- `print(...args)`: Mostrar no console
- `channel()`: Criar canal de comunicação
- `send(sender, value)`: Enviar valor
- `recv(receiver)`: Receber valor
- `spawn { ... }`: Iniciar thread
- `Qubit()`: Criar qubit
- `H(qubit)`: Hadamard gate
- `Measure(qubit)`: Medir qubit

---

## Exemplos

### Padrão Produtor-Consumidor

```ssl
fn main() {
    let chan = channel()
    
    spawn {
        for i in 0..10 {
            send(chan[0], i)
        }
    }
    
    for i in 0..10 {
        print("Recebido:", recv(chan[1]))
    }
}
```

---

## FAQ

**SSL está pronto para produção?**
Não, SSL é experimental.

**Quais plataformas são suportadas?**
Todas as plataformas Rust (Windows, macOS, Linux).

---

**Construído com ❤️ e Rust** 🦀
