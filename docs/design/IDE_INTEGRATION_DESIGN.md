# SSL IDE Integration - Design Document

**Version**: 1.0  
**Status**: Design Phase  
**Target Release**: Phase 9 (Q1 2026)

---

## 1. Overview

### 1.1 Supported IDEs

Priority order:
1. **VS Code** (highest priority - largest user base)
2. **IntelliJ IDEA / CLion** (professional developers)
3. **Vim/Neovim** (LSP client)
4. **Emacs** (LSP client)

### 1.2 Core Features

- ✅ Syntax highlighting
- ✅ Auto-completion (IntelliSense)
- ✅ Go-to-definition / Find references
- ✅ Real-time error diagnostics
- ✅ Code formatting
- ✅ Refactoring support
- ✅ Debugger integration ⏰
- ✅ Quantum circuit visualization ⚛️

---

## 2. LSP Enhancements

### 2.1 Enhanced Capabilities

```typescript
// Current LSP capabilities (Phase 7)
const capabilities = {
    textDocumentSync: TextDocumentSyncKind.Full,
    completionProvider: {},
    hoverProvider: true,
};

// NEW Phase 9 capabilities
const enhancedCapabilities = {
    ...capabilities,
    
    // Go-to-definition
    definitionProvider: true,
    
    // Find all references
    referencesProvider: true,
    
    // Rename symbol
    renameProvider:  true,
    
    // Code actions (quick fixes)
    codeActionProvider: {
        codeActionKinds: [
            CodeActionKind.QuickFix,
            CodeActionKind.Refactor,
            CodeActionKind.Source,
        ]
    },
    
    // Document formatting
    documentFormattingProvider: true,
    documentRangeFormattingProvider: true,
    
    // Inlay hints (type annotations)
    inlayHintProvider: true,
    
    // Semantic tokens (better highlighting)
    semanticTokensProvider: {
        legend: {
            tokenTypes: ['variable', 'function', 'class', 'keyword', ...],
            tokenModifiers: ['declaration', 'definition', 'readonly', ...]
        },
        full: true,
    },
    
    // Code lens (inline actions)
    codeLensProvider: {
        resolveProvider: true
    },
};
```

### 2.2 New LSP Features

#### 2.2.1 Inlay Hints (Type Annotations)

```ssl
// User sees:
let data = fetch_data()
       ↑: HttpResponse

fn process(items) -> Result {
          ↑: List<Item>
```

#### 2.2.2 Code Actions (Quick Fixes)

```ssl
// User code:
let x = undefined_var
       ~~~~~~~~~~~~~ 
       💡 Did you mean 'undefined_variable'?
          ⚡ Create variable 'undefined_var'
```

#### 2.2.3 Code Lens (Run/Debug buttons)

```ssl
fn fibonacci(n: Int) -> Int {  // ▶️ Run | 🐛 Debug | 📊 Benchmark
    if n <= 1 { return n }
    return fibonacci(n - 1) + fibonacci(n - 2)
}
```

---

## 3. VS Code Extension Enhancements

### 3.1 New Features

```json
// package.json
{
  "name": "ssl-vscode",
  "version": "2.0.0",
  "contributes": {
    "commands": [
      {
        "command": "ssl.runFile",
        "title": "SSL: Run File"
      },
      {
        "command": "ssl.debugFile",
        "title": "SSL: Debug with Time-Travel"
      },
      {
        "command": "ssl.visualizeQuantum",
        "title": "SSL: Visualize Quantum Circuit"
      },
      {
        "command": "ssl.aiReview",
        "title": "SSL: AI Code Review"
      }
    ],
    
    "debuggers": [{
      "type": "ssl-debug",
      "label": "SSL Time-Travel Debugger",
      "program": "./debugger-adapter.js"
    }],
    
    "viewsContainers": {
      "activitybar": [{
        "id": "ssl-explorer",
        "title": "SSL",
        "icon": "resources/ssl-icon.svg"
      }]
    },
    
    "views": {
      "ssl-explorer": [
        {
          "id": "sslPackages",
          "name": "Packages"
        },
        {
          "id": "sslQuantumCircuits",
          "name": "Quantum Circuits"
        }
      ]
    }
  }
}
```

### 3.2 Debugger Integration ⏰

**Debug Adapter Protocol (DAP)**:

```typescript
// debugger-adapter.ts

class SSLDebugAdapter implements DebugProtocol.DebugAdapter {
    async launch(args: LaunchRequestArguments) {
        // Start SSL with --debug flag
        this.sslProcess = spawn('ssl', ['run', args.program, '--debug']);
        
        // Connect to debug UI
        this.debugUI = await connectToDebugUI();
    }
    
    async setBreakpoints(args: SetBreakpointsArguments) {
        // Set breakpoints in SSL debugger
    }
    
    async continue() {
        await this.debugUI.send({ command: '@forward' });
    }
    
    async stepBack() {
        // ⏰ TIME-TRAVEL: Unique to SSL!
        await this.debugUI.send({ command: '@back' });
    }
    
    async stackTrace() {
        return await this.debugUI.send({ command: '@inspect' });
    }
}
```

### 3.3 Quantum Circuit Visualization ⚛️

```typescript
// quantum-visualizer.ts

export class QuantumCircuitPanel {
    async visualize(code: string) {
        // Parse SSL quantum code
        const ast = await parse(code);
        const circuit = extractQuantumCircuit(ast);
        
        // Render as SVG
        const svg = renderCircuit(circuit);
        
        // Show in webview
        this.panel.webview.html = `
            <html>
                <body>
                    <h1>Quantum Circuit</h1>
                    ${svg}
                    <div>Qubits: ${circuit.numQubits}</div>
                    <div>Gates: ${circuit.gates.length}</div>
                </body>
            </html>
        `;
    }
}
```

**Example visualization:**

```
q0: ──H─────●─────
            │
q1: ────────X─────
```

---

## 4. IntelliJ IDEA Plugin

### 4.1 Architecture

```
SSL IDEA Plugin
├── Language Support
│   ├── SSLFileType
│   ├── SSLLanguage
│   └── SSLParserDefinition
├── Syntax Highlighting
│   └── SSLSyntaxHighlighter
├── Code Completion
│   └── SSLCompletionContributor
├── Refactoring
│   ├── RenameRefactoring
│   └── ExtractFunctionRefactoring
└── External Tools Integration
    ├── SSL Compiler
    └── sslpkg Integration
```

### 4.2 Key Features

```kotlin
// SSLCompletionContributor.kt
class SSLCompletionContributor : CompletionContributor() {
    override fun fillCompletionVariants(
        parameters: CompletionParameters,
        result: CompletionResultSet
    ) {
        // LSP-based completion
        val lspCompletions = lspClient.completion(
            TextDocumentIdentifier(parameters.position.file.url),
            parameters.position
        )
        
        lspCompletions.forEach { item ->
            result.addElement(
                LookupElementBuilder.create(item.label)
                    .withTypeText(item.detail)
                    .withIcon(SSLIcons.forKind(item.kind))
            )
        }
    }
}
```

---

## 5. Vim/Neovim Integration

### 5.1 nvim-lspconfig

```lua
-- nvim/lua/ssl.lua

require('lspconfig').ssl_lsp.setup {
    cmd = { 'ssl', 'lsp' },
    filetypes = { 'ssl' },
    root_dir = function(fname)
        return vim.fn.getcwd()
    end,
    settings = {
        ssl = {
            checkOnSave = true,
            allFeatures = true,
        }
    },
    on_attach = function(client, bufnr)
        -- Keybindings
        vim.api.nvim_buf_set_keymap(bufnr, 'n', 'gd', '<cmd>lua vim.lsp.buf.definition()<CR>', {})
        vim.api.nvim_buf_set_keymap(bufnr, 'n', 'K', '<cmd>lua vim.lsp.buf.hover()<CR>', {})
        vim.api.nvim_buf_set_keymap(bufnr, 'n', '<leader>r', '<cmd>lua vim.lsp.buf.rename()<CR>', {})
    end
}
```

### 5.2 Syntax Highlighting

```vim
" ssl.vim

syn keyword sslKeyword fn let mut if else for while return spawn try recover
syn keyword sslType Int String Bool List Qubit
syn keyword sslBoolean true false
syn keyword sslQuantum H X CNOT Measure Qubit

syn match sslFunction "\w\+\s*("
syn match sslOperator "[-+*/%=<>!&|]"
syn region sslString start='"' end='"'
syn match sslNumber '\d\+'

hi def link sslKeyword Keyword
hi def link sslType Type
hi def link sslFunction Function
```

---

## 6. Implementation Roadmap

### Phase 9.1: LSP Enhancements (Week 1-3)

- [ ] Go-to-definition
- [ ] Find references
- [ ] Rename refactoring
- [ ] Code actions (quick fixes)

### Phase 9.2: VS Code Extension (Week 4-6)

- [ ] Debugger integration (DAP)
- [ ] Quantum circuit visualizer
- [ ] Package explorer view
- [ ] Run configurations

### Phase 9.3: IntelliJ Plugin (Week 7-9)

- [ ] Language support
- [ ] Syntax highlighting
- [ ] Code completion
- [ ] Refactoring

### Phase 9.4: Vim/Emacs (Week 10)

- [ ] LSP configuration
- [ ] Syntax files
- [ ] Documentation

---

## 7. Code Stubs

See `src/lsp/enhancements.rs` and `editors/vscode/` for stubs.

---

## 8. References

- **LSP Specification**: https://microsoft.github.io/language-server-protocol/
- **DAP Specification**: https://microsoft.github.io/debug-adapter-protocol/
- **VS Code API**: https://code.visualstudio.com/api
- **IntelliJ Platform SDK**: https://plugins.jetbrains.com/docs/intellij/
