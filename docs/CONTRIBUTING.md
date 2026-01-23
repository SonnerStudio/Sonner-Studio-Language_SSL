# Contributing to Sonner Studio Language (SSL)

Thank you for your interest in contributing to SSL! This document provides guidelines for contributing.

## Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR-USERNAME/SSL.git`
3. Create a branch: `git checkout -b feature/my-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Commit: `git commit -m "Add my feature"`
7. Push: `git push origin feature/my-feature`
8. Create a Pull Request

## Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/SonnerStudio/SSL.git
cd SSL
cargo build

# Run tests
cargo test

# Run the CLI
cargo run -- doctor
```

## Project Structure

```
src/
├── lib.rs           # Library root (31 modules)
├── main.rs          # CLI entry point
├── cli.rs           # Command definitions
├── lexer.rs         # Tokenization
├── parser.rs        # AST generation
├── interpreter.rs   # Execution
├── ast.rs           # AST types
├── lsp/             # Language Server
├── ai/              # AI Assistant
├── wasm/            # WebAssembly
├── mobile/          # iOS/Android
├── edge/            # Edge deployment
├── reactive/        # Reactive streams
├── crdt/            # CRDT types
├── gpu/             # GPU/SIMD
├── verify/          # Verification
├── effects/         # Algebraic effects
├── linear/          # Linear types
├── property_test/   # Property testing
└── ...
```

## Contribution Areas

### High Priority (v4.1 planning)
- [ ] Full LLVM codegen
- [ ] iOS/Android build integration
- [ ] CRDT network sync
- [ ] GPU kernel execution
- [ ] Effect handlers

### Beginner-Friendly
- Documentation improvements
- Example files
- Test coverage
- Error messages

### Advanced
- Parser extensions
- Type checker improvements
- Optimizer passes
- Platform backends

## Coding Guidelines

### Rust Style
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Follow Rust API guidelines

### SSL Style
- 4 spaces for indentation
- `snake_case` for functions and variables
- `PascalCase` for types and structs
- Document public APIs

### Commit Messages
```
<type>(<scope>): <description>

Types: feat, fix, docs, style, refactor, test, chore
Example: feat(parser): add support for effect declarations
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_parser

# Run with verbose output
cargo test -- --nocapture
```

## Documentation

- Update docs/ for user-facing features
- Add doc comments for public APIs
- Include examples in documentation

## Pull Request Process

1. Ensure all tests pass
2. Update documentation
3. Add entry to CHANGELOG (if applicable)
4. Request review from maintainers
5. Address feedback
6. Merge after approval

## Questions?

- Open an issue for bugs or features
- Use discussions for questions
- Email: ssl@sonnerstudio.com

---

Thank you for contributing to SSL! 🎉
