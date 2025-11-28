# Sonner Studio Language (SSL)

![SSL Logo](assets/Sonner-Studio-Language-Logo.png)

**Uma linguagem de programação universal nativa de IA do futuro**

[![Versão](https://img.shields.io/badge/versão-0.1.0-blue.svg)](https://github.com/yourusername/ssl)
[![Licença](https://img.shields.io/badge/licença-MIT-green.svg)](LICENSE)
[![Testes](https://img.shields.io/badge/testes-aprovados-brightgreen.svg)](#tests)

---

## 🌍 Idiomas / Languages

**[English](README.md)** | **[Deutsch](README_DE.md)** | **[Français](README_FR.md)** | **[Español](README_ES.md)** | **[Português](README_PT.md)** | **[日本語](README_JA.md)**

---

## 🌟 Destaques

SSL é uma **linguagem de programação experimental** que combina conceitos modernos e futuristas:

- ⚡ **Paralelo por Design**: Suporte nativo para threads e passagem de mensagens
- ⚛️ **Computação Quântica**: Simulador quântico integrado
- 🩹 **Código Auto-Reparável**: Tratamento automático de erros com integração de IA
- 🤖 **Nativo de IA**: Compilador com otimização e análise de erros por IA
- 🔄 **Sistema de Tipos Híbrido**: Estático + Dinâmico + Inferência

---

## 🚀 Início Rápido

### Instalação

```bash
git clone https://github.com/yourusername/ssl.git
cd ssl
cargo build --release
```

### Seu primeiro programa SSL

```ssl
fn main() {
    print("Olá, SSL!")
}
```

Executar:
```bash
ssl run examples/hello.ssl
```

---

## 📖 Funcionalidades

### 1. Paralelismo ⚡

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
H(q)  // Hadamard: Superposição
let result = Measure(q)
print(result)  // 0 ou 1 (50/50)
```

### 3. Código Auto-Reparável 🩹

```ssl
try {
    let result = operacao_arriscada()
} recover (err) {
    print("Erro capturado:", err)
    // Recuperação automática
}
```

### 4. Funções & Recursão

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
ssl run <arquivo>   # Executar código
ssl build <caminho> # Compilar projeto
ssl check <arquivo> # Verificar sintaxe
ssl doctor          # Verificação do sistema
ssl lsp             # Iniciar servidor de linguagem
```

### Daemon de IA (ssld)

```bash
ssld  # Inicia o daemon de IA para análise de código
```

### Extensão VS Code

1. Abra `editors/vscode/`
2. `npm install`
3. `npm run compile`
4. F5 para depurar

---

## 🧪 Testes

```bash
cargo test
```

**Status**: Todos os 9 testes unitários aprovados ✅

- ✅ Aritmética & Variáveis
- ✅ Funções & Recursão
- ✅ Operadores de Comparação
- ✅ Threads (`spawn`)
- ✅ Canais (`channel`, `send`, `recv`)
- ✅ Gates Quânticos (`Qubit`, `H`, `Measure`)
- ✅ Try-Recover (Auto-Reparação)

---

## 📚 Documentação

- [Plano Mestre de Implementação](docs/implementation_master.md)
- [Fase 4: Ferramentas](docs/implementation_phase4.md)
- [Fase 5: Funcionalidades Avançadas](docs/implementation_phase5.md)
- [Guia Prático](brain/walkthrough.md)

---

## 🗺️ Roadmap

### Fase 0-5: ✅ Concluída
- [x] Visão & Filosofia
- [x] Arquitetura Central (EBNF, Sistema de Tipos)
- [x] Protótipo do Interpretador
- [x] Integração de IA (Ollama)
- [x] Ferramentas (CLI, LSP, Daemon de IA)
- [x] Funcionalidades Avançadas (Paralelo, Quântico, Auto-Reparação)

### Fase 6: 🚧 Em Planejamento
- [ ] Comunidade & Evolução Aberta
- [ ] Modelo de Governança
- [ ] Documentação & Tutoriais

### Fase 7: 🔮 Visão de Longo Prazo
- [ ] Conceitos de Compilador Evolucionário
- [ ] Código Auto-Modificável
- [ ] Computação Distribuída

---

## 🤝 Contribuições

SSL é um projeto experimental. Contribuições são bem-vindas!

1. Fork o projeto
2. Crie um branch de recurso (`git checkout -b feature/amazing`)
3. Faça commit das suas alterações (`git commit -m 'Add amazing feature'`)
4. Faça push para o branch (`git push origin feature/amazing`)
5. Abra um Pull Request

---

## 📄 Licença

Licença MIT - veja [LICENSE](LICENSE) para detalhes.

---

## 🙏 Agradecimentos

- **Comunidade Rust** pelo excelente suporte de ferramentas
- **Ollama** pelo framework de IA
- **tower-lsp** pela integração LSP

---

## 📧 Contato

- GitHub: [@yourusername](https://github.com/yourusername)
- Email: your.email@example.com
- Discord: Servidor da Comunidade SSL (em breve)

---

**Construído com ❤️ e Rust** 🦀
