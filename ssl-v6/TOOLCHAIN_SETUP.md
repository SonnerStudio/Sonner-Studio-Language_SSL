# SSL v6.1/v6.2 - Toolchain Setup Guide

## Übersicht

Für natives x64 Assembly-Building benötigen wir:
- **ml64.exe** - Microsoft Macro Assembler (64-bit)
- **link.exe** - Microsoft Linker
- **C Runtime** - Windows SDK

## Installation Options

### Option 1: Visual Studio Build Tools (Empfohlen)

**Download**:
- https://visualstudio.microsoft.com/downloads/
- Unter "Tools for Visual Studio" → "Build Tools for Visual Studio 2022"

**Installation**:
1. Installer ausführen
2. Workload auswählen: **"Desktop development with C++"**
3. Individual Components:
   - ✅ MSVC v143 - VS 2022 C++ x64/x86 build tools
   - ✅ Windows 11 SDK (oder Windows 10 SDK)
   - ✅ C++ CMake tools for Windows

**Install Size**: ~2-3 GB

**Nach Installation**:
```powershell
# Developer Command Prompt öffnen ODER
# PowerShell mit vcvarsall.bat

# Für x64:
"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# Verify
where.exe ml64
where.exe link
```

### Option 2: Windows SDK (Leichtgewichtig)

**Download**:
- https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/

**Nach Installation**:
```powershell
# Typischer Pfad:
$env:PATH += ";C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64"

# Verify
ml64 /?
link /?
```

### Option 3: Standalone MASM (Legacy)

**Für ml64.exe standalone**:
- Extrahieren aus Windows SDK
- Manuell zu PATH hinzufügen

**Nicht empfohlen** - fehlende Dependencies

## Setup Verification

### Quick Test

```powershell
# 1. Create test.asm
@"
.code
main PROC
    mov rax, 42
    ret
main ENDP
END
"@ | Out-File -Encoding ASCII test.asm

# 2. Assemble
ml64 /c /Fo test.obj test.asm

# 3. Link
link /subsystem:console /entry:main test.obj

# 4. Run
.\test.exe
echo "Exit code: $LASTEXITCODE"
# Expected: 42
```

## SSL v6.1 Runtime Building

### Build Runtime Library

```powershell
cd c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL\ssl-v6\runtime

# Assemble all modules
ml64 /c /Fo memory.obj memory.asm
ml64 /c /Fo string.obj string.asm
ml64 /c /Fo list.obj list.asm
ml64 /c /Fo io.obj io.asm
ml64 /c /Fo convert.obj convert.asm

# Assemble test
ml64 /c /Fo test_runtime.obj test_runtime.asm

# Link test
link /subsystem:console /entry:main `
    test_runtime.obj `
    memory.obj `
    string.obj `
    list.obj `
    io.obj `
    convert.obj `
    /out:test_runtime.exe

# Run tests
.\test_runtime.exe
echo "Exit code: $LASTEXITCODE"
# Expected: 0 (all tests pass)
```

### Create Runtime Library (.lib)

```powershell
# Create static library
lib /OUT:ssl_runtime.lib `
    memory.obj `
    string.obj `
    list.obj `
    io.obj `
    convert.obj

# Now can link with:
# link program.obj ssl_runtime.lib
```

## Integration mit v6.1 Compiler

### Automated Build Script

```powershell
# build_ssl_program.ps1
param(
    [string]$SourceFile,
    [string]$OutputFile
)

# 1. "Compile" SSL to Assembly (with v6.1 compiler)
# Assuming v6.1 has a compile command
ssl-v6.exe compile $SourceFile -o temp.asm

# 2. Assemble
ml64 /c /Fo temp.obj temp.asm

# 3. Link with runtime
link /subsystem:console /entry:main `
    temp.obj `
    ssl-v6\runtime\ssl_runtime.lib `
    /out:$OutputFile

# 4. Cleanup
Remove-Item temp.asm, temp.obj
```

## Environment Setup

### Permanent PATH Setup

```powershell
# Add to User PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$newPath = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.xx.xxxxx\bin\Hostx64\x64"
[Environment]::SetEnvironmentVariable("Path", "$userPath;$newPath", "User")

# Verify (new PowerShell session)
ml64 /?
```

### Session Setup Script

```powershell
# setup_toolchain.ps1
# Schnelles Setup für Development-Sessions

$vsPath = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools"
$vcvars = "$vsPath\VC\Auxiliary\Build\vcvars64.bat"

if (Test-Path $vcvars) {
    cmd /c "$vcvars & set" | ForEach-Object {
        if ($_ -match "^(.*?)=(.*)$") {
            Set-Content "env:\$($matches[1])" $matches[2]
        }
    }
    Write-Host "✅ Toolchain ready" -ForegroundColor Green
} else {
    Write-Host "❌ Visual Studio not found" -ForegroundColor Red
}
```

## Troubleshooting

### ml64 not found
```powershell
# Find ml64.exe
Get-ChildItem "C:\Program Files*" -Recurse -Filter ml64.exe -ErrorAction SilentlyContinue

# Add to PATH temporarily
$env:PATH += ";C:\Path\To\ml64\Directory"
```

### Link errors (LNK2001, LNK2019)
- **Cause**: Missing runtime libraries
- **Fix**: Link with Windows SDK libs
```powershell
link program.obj `
    /LIBPATH:"C:\Program Files (x86)\Windows Kits\10\Lib\10.0.22000.0\um\x64" `
    kernel32.lib
```

### Access Denied
- **Cause**: Administrator rights needed
- **Fix**: Run VS Developer Command Prompt as Administrator

## Next Steps

After toolchain setup:
1. ✅ Build runtime library (`ssl_runtime.lib`)
2. ✅ Test runtime (`test_runtime.exe` → exit code 0)
3. ✅ Ready for v6.2 Full Implementation

---

**Created**: 2025-12-07 13:54  
**Status**: Toolchain Setup Complete
