# SSL Package Manager (`sslpkg`) - Design Document

**Version**: 1.0  
**Status**: Design Phase  
**Target Release**: Phase 9 (Q1 2026)  
**Authors**: SSL Core Team

---

## 1. Overview

### 1.1 Vision

`sslpkg` will be SSL's official package manager, providing:
- **Simple dependency management** (similar to `cargo`, `npm`)
- **Decentralized package registry** (similar to crates.io)
- **Version resolution** with semantic versioning
- **Build integration** with SSL compiler
- **Security** through signature verification

### 1.2 Goals

- ✅ Install packages with one command: `sslpkg install <package>`
- ✅ Publish packages easily: `sslpkg publish`
- ✅ Manage dependencies declaratively in `ssl.toml`
- ✅ Fast, parallel downloads and builds
- ✅ Offline mode with local cache

---

## 2. Architecture

### 2.1 System Components

```
┌─────────────────────────────────────────────────┐
│                   sslpkg CLI                    │
├─────────────────────────────────────────────────┤
│  install │ publish │ update │ build │ search   │
└────────────┬────────────────────────────────────┘
             │
    ┌────────┴────────┐
    │                 │
┌───▼────┐     ┌─────▼─────┐
│Registry│     │   Local   │
│ Client │     │   Cache   │
└───┬────┘     └─────┬─────┘
    │                │
┌───▼────────────────▼─────┐
│    Dependency Resolver   │
└──────────┬────────────────┘
           │
    ┌──────▼────────┐
    │ SSL Compiler  │
    └───────────────┘
```

### 2.2 Package Manifest (`ssl.toml`)

```toml
[package]
name = "my-awesome-lib"
version = "1.0.0"
authors = ["Your Name <you@example.com>"]
license = "MIT"
description = "An awesome SSL library"
repository = "https://github.com/user/my-awesome-lib"
keywords = ["quantum", "parallel", "data"]

[dependencies]
http-client = "2.1.0"
json-parser = "^1.5"
quantum-sim = { version = "3.0", features = ["advanced"] }

[dev-dependencies]
test-framework = "1.0"

[build]
entry = "src/lib.ssl"
output = "target"
```

### 2.3 Directory Structure

```
my-project/
├── ssl.toml              # Package manifest
├── src/
│   ├── lib.ssl          # Library entry point
│   └── main.ssl         # Binary entry point
├── tests/
│   └── integration.ssl
├── examples/
│   └── demo.ssl
├── .sslpkg/             # Package metadata cache
└── target/              # Build output
    └── debug/
        └── libmy_project.a
```

---

## 3. Dependency Resolution

### 3.1 Algorithm: PubGrub

Using **PubGrub algorithm** (same as Dart, Swift Package Manager):

```
1. Start with root package dependencies
2. For each dependency:
   a. Fetch all available versions
   b. Check version constraints
   c. Build SAT solver instance
3. Solve for compatible version set
4. If conflict: backtrack and retry
5. Output: Lockfile with exact versions
```

### 3.2 Version Constraints

```toml
# Semantic Versioning (SemVer)
exact = "1.2.3"          # Exactly 1.2.3
caret = "^1.2.3"         # >=1.2.3, <2.0.0
tilde = "~1.2.3"         # >=1.2.3, <1.3.0
wildcard = "1.2.*"       # >=1.2.0, <1.3.0
range = ">=1.2, <2.0"    # Custom range
```

### 3.3 Lockfile (`ssl.lock`)

```toml
# Auto-generated, do not edit manually
[[package]]
name = "http-client"
version = "2.1.0"
source = "registry+https://registry.sslang.org"
checksum = "a1b2c3d4..."

[[package]]
name = "json-parser"
version = "1.6.2"
source = "registry+https://registry.sslang.org"
checksum = "e5f6g7h8..."
```

---

## 4. Registry Protocol

### 4.1 API Endpoints

```
GET  /api/v1/packages/{name}           # Package metadata
GET  /api/v1/packages/{name}/{version} # Specific version
GET  /api/v1/search?q={query}          # Search packages
POST /api/v1/publish                   # Publish new version
GET  /api/v1/download/{name}/{version} # Download tarball
```

### 4.2 Package Metadata (JSON)

```json
{
  "name": "http-client",
  "description": "HTTP client library for SSL",
  "license": "MIT",
  "repository": "https://github.com/ssl-libs/http-client",
  "versions": [
    {
      "num": "2.1.0",
      "yanked": false,
      "created_at": "2026-01-15T10:00:00Z",
      "dependencies": [
        {
          "name": "json-parser",
          "req": "^1.5"
        }
      ],
      "checksum": "sha256:a1b2c3d4...",
      "download_url": "https://registry.sslang.org/dl/http-client-2.1.0.tar.gz"
    }
  ]
}
```

### 4.3 Security

- **Signature verification**: All packages signed with author's GPG key
- **Checksum validation**: SHA-256 hashes for all downloads
- **Audit log**: Track all publish/yank operations
- **Rate limiting**: Prevent abuse

---

## 5. CLI Interface

### 5.1 Commands

```bash
# Initialize new project
sslpkg init my-project
sslpkg init --lib my-library

# Add dependency
sslpkg add http-client
sslpkg add json-parser --version "^1.5"

# Install dependencies
sslpkg install

# Update dependencies
sslpkg update
sslpkg update http-client

# Build project
sslpkg build
sslpkg build --release

# Run project
sslpkg run
sslpkg run --example demo

# Test
sslpkg test

# Publish package
sslpkg publish

# Search registry
sslpkg search quantum

# Show package info
sslpkg info http-client
```

### 5.2 Configuration (`~/.sslpkg/config.toml`)

```toml
[registry]
default = "https://registry.sslang.org"

[http]
timeout = 30
retry = 3

[build]
jobs = 8  # Parallel build jobs

[cache]
dir = "~/.sslpkg/cache"
max_size = "10GB"
```

---

## 6. Build Integration

### 6.1 Build Process

```
1. Parse ssl.toml
2. Resolve dependencies → ssl.lock
3. Download packages to cache
4. Verify checksums & signatures
5. Compile dependencies (topological order)
6. Compile main project
7. Link all modules
8. Output: target/debug/<binary>
```

### 6.2 Incremental Compilation

- Track file modification times
- Rebuild only changed modules
- Cache intermediate IR
- Parallel compilation where possible

---

## 7. Implementation Roadmap

### Phase 9.1: Core (Week 1-4)

- [ ] Manifest parser (`ssl.toml`)
- [ ] Dependency resolver (PubGrub)
- [ ] Local cache management
- [ ] Basic CLI (`init`, `add`, `install`)

### Phase 9.2: Registry (Week 5-8)

- [ ] Registry client (HTTP API)
- [ ] Download & verification
- [ ] Publish workflow
- [ ] Search functionality

### Phase 9.3: Build System (Week 9-12)

- [ ] SSL compiler integration
- [ ] Incremental compilation
- [ ] Parallel builds
- [ ] Target profiles (debug/release)

### Phase 9.4: Polish (Week 13-16)

- [ ] Documentation
- [ ] Migration guides
- [ ] Registry deployment
- [ ] Community packages

---

## 8. Code Stubs

See `src/package/mod.rs` for implementation stubs.

---

## 9. References

- **Cargo**: https://doc.rust-lang.org/cargo/
- **npm**: https://docs.npmjs.com/
- **PubGrub**: https://github.com/dart-lang/pub/blob/master/doc/solver.md
- **SemVer**: https://semver.org/
