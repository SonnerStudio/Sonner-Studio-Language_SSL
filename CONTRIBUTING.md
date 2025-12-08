# Contributing to SSL v7.0

Thank you for your interest in contributing to Sonner Studio Language! This document provides guidelines for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Pull Request Process](#pull-request-process)
- [Reporting Bugs](#reporting-bugs)
- [Feature Requests](#feature-requests)

## Code of Conduct

This project adheres to a Code of Conduct that all contributors are expected to follow. Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before contributing.

## Getting Started

### Prerequisites

- Rust 1.70+ (for building the compiler)
- NASM assembler
- Git
- Familiarity with SSL syntax

### Fork and Clone

```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/YOUR_USERNAME/SSL.git
cd SSL

# Add upstream remote
git remote add upstream https://github.com/SonnerStudio/SSL.git
```

## How to Contribute

### Types of Contributions

We welcome:

- **Bug fixes** - Fix issues in the compiler, runtime, or documentation
- **Features** - Implement new language features or compiler optimizations
- **Documentation** - Improve guides, tutorials, or API documentation
- **Tests** - Add test cases for better coverage
- **Examples** - Create example programs demonstrating SSL features
- **Translations** - Translate documentation to other languages

## Development Setup

### Build from Source

```bash
# Install dependencies
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

### Running the Compiler

```bash
# Test your changes
ssl run examples/hello.ssl

# Run compiler tests
cargo test --package ssl-compiler
```

### Project Structure

```
SSL/
├── src/              # Rust CLI wrapper
├── ssl-v7/          # SSL v7.0 compiler (written in SSL)
│   ├── src/         # Compiler source
│   │   ├── lexer.ssl
│   │   ├── parser.ssl
│   │   ├── ir.ssl
│   │   └── codegen.ssl
│   └── tests/       # Compiler tests
├── docs/            # Documentation
├── examples/        # Example programs
└── tests/           # Integration tests
```

## Coding Standards

### SSL Code Style

```ssl
// Use descriptive names
fn calculate_fibonacci(n: Int) -> Int {
    // Clear comments for complex logic
    if n <= 1 {
        return n
    } else {
        return calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
    }
}

// Prefer explicit types in public APIs
fn public_api(param: Int) -> String {
    return int_to_string(param)
}
```

### Rust Code Style

Follow standard Rust conventions:
- Use `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Write doc comments for public APIs

```rust
/// Compiles an SSL source file to native code.
///
/// # Arguments
/// * `source_path` - Path to the .ssl file
/// * `output_path` - Path for the output executable
///
/// # Returns
/// Result indicating success or compilation error
pub fn compile(source_path: &Path, output_path: &Path) -> Result<(), CompileError> {
    // Implementation
}
```

### Documentation

- **Code comments**: Explain *why*, not *what*
- **Doc comments**: Use for public APIs
- **Markdown docs**: Follow existing structure

## Pull Request Process

### Before Submitting

1. **Update your fork**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Create a feature branch**:
   ```bash
   git checkout -b feature/my-new-feature
   ```

3. **Make your changes**:
   - Write clear, focused commits
   - Add tests for new functionality
   - Update documentation

4. **Test thoroughly**:
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy
   ```

5. **Commit with clear messages**:
   ```bash
   git commit -m "feat: Add ARM64 SIMD optimization
   
   - Implement vectorized operations for ARM64
   - Add tests for new SIMD instructions
   - Update documentation
   
   Closes #123"
   ```

### Commit Message Format

Use conventional commits:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(compiler): Add constant folding optimization

Implement compile-time evaluation of constant expressions.
This reduces runtime overhead for mathematical constants.

Closes #456
```

```
fix(lexer): Handle Unicode escape sequences correctly

Previously, \u{XXXX} sequences were not parsed correctly.
This fix implements proper Unicode handling.

Fixes #789
```

### Pull Request Template

When opening a PR, include:

```markdown
## Description
Brief description of changes

## Motivation
Why is this change needed?

## Changes
- List of specific changes
- Another change

## Testing
How was this tested?

## Checklist
- [ ] Tests pass (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (for user-facing changes)
```

### Review Process

1. **Automated checks** must pass (CI/CD)
2. **Code review** by maintainers
3. **Testing** on multiple platforms
4. **Approval** and merge

## Reporting Bugs

### Before Reporting

- Search existing issues
- Verify with latest version
- Minimal reproduction case

### Bug Report Template

```markdown
**Description**
Clear description of the bug

**To Reproduce**
Steps to reproduce:
1. Create file `test.ssl` with...
2. Run `ssl compile test.ssl`
3. See error

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Environment**
- SSL Version: v7.0.0
- OS: Windows 11 / Ubuntu 22.04
- Architecture: x86_64

**Additional Context**
- Error messages
- Compiler output
- Generated assembly (if applicable)
```

## Feature Requests

### Template

```markdown
**Feature Description**
Clear description of the proposed feature

**Motivation**
Why is this feature needed?

**Proposed Solution**
How should it work?

**Alternatives Considered**
Other approaches you've considered

**Additional Context**
Examples, use cases, etc.
```

## Areas Needing Help

Current priorities:

1. **ARM64 HAL Implementation** - Hardware abstraction for ARM platforms
2. **Standard Library** - Expand built-in functions
3. **Documentation** - More tutorials and examples
4. **Testing** - Increase test coverage
5. **Optimization** - Compiler performance improvements

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Credited in documentation

## Questions?

- **Discord**: [Join our community](https://discord.gg/sonnerstudio)
- **Discussions**: [GitHub Discussions](https://github.com/SonnerStudio/SSL/discussions)
- **Email**: dev@sonnerstudio.com

---

**Thank you for contributing to SSL!** 🚀
