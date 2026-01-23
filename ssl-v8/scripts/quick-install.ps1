# Quick SSL v8 Local Installation Script
# Installs the locally built SSL binary

$ErrorActionPreference = "Stop"

Write-Host "SSL v8.0 Quick Installer" -ForegroundColor Green
Write-Host "========================" -ForegroundColor Green
Write-Host ""

# Get script directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent (Split-Path -Parent $ScriptDir)
$BinaryPath = Join-Path $ProjectRoot "ssl-v8\binaries\ssl-windows-x64.exe"

# Check if binary exists
if (-not (Test-Path $BinaryPath)) {
    Write-Host "Error: Binary not found at $BinaryPath" -ForegroundColor Red
    Write-Host "Please build the project first with 'cargo build --release'" -ForegroundColor Yellow
    exit 1
}

# Verify binary version
Write-Host "Found binary at: $BinaryPath"
Write-Host "Version: " -NoNewline
& $BinaryPath --version
Write-Host ""

# Installation directory
$InstallDir = "$env:LOCALAPPDATA\Programs\SSL"

# Create directory
if (-not (Test-Path $InstallDir)) {
    Write-Host "Creating installation directory: $InstallDir"
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Copy binary
Write-Host "Installing SSL to: $InstallDir\ssl.exe"
Copy-Item -Path $BinaryPath -Destination "$InstallDir\ssl.exe" -Force

# Add to PATH
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    Write-Host "Adding SSL to user PATH..."
    $NewPath = "$UserPath;$InstallDir"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "  PATH updated for current user" -ForegroundColor Green
    Write-Host "  Please restart your terminal to use 'ssl' command" -ForegroundColor Yellow
}
else {
    Write-Host "SSL is already in PATH" -ForegroundColor Green
}

# Update current session PATH
$env:Path = "$env:Path;$InstallDir"

Write-Host ""
Write-Host "✓ Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Testing SSL command..."
try {
    ssl --version
    Write-Host ""
    Write-Host "✓ SSL is working correctly" -ForegroundColor Green
    Write-Host ""
    Write-Host "Run 'ssl --help' to get started!" -ForegroundColor Cyan
}
catch {
    Write-Host ""
    Write-Host "ℹ SSL installed successfully" -ForegroundColor Green
    Write-Host "  Restart your terminal and run 'ssl --help' to get started!" -ForegroundColor Cyan
}
