# Quick Installation

```bash
# Install SSL v7.0 via cargo
cargo install --git https://github.com/SonnerStudio/SSL

# Verify
ssl --version

# Create your first program
echo 'fn main() -> Int { print("Hello, SSL!") return 0 }' > hello.ssl

# Compile and run
ssl compile hello.ssl
./hello
```

See [INSTALLATION.md](docs/INSTALLATION.md) for detailed instructions.
