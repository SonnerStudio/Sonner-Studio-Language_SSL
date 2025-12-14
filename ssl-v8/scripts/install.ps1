# SSL v8.0 Installer Script for Windows
# Usage: irm https://raw.githubusercontent.com/SonnerStudio/SSL-v8/main/scripts/install.ps1 | iex

param(
    [switch]$DryRun
)

# Configuration
$SSLVersion = "8.0.0"
$GitHubRepo = "SonnerStudio/SSL-v8"
$InstallDir = "$env:ProgramFiles\SSL"
$BinaryName = "ssl-windows-x64.exe"

Write-Host "SSL v$SSLVersion Installer" -ForegroundColor Green
Write-Host "==========================" -ForegroundColor Green
Write-Host ""

# Detect Architecture
$Arch = [System.Environment]::Is64BitOperatingSystem
if (-not $Arch) {
    Write-Host "Error: Only 64-bit Windows is supported" -ForegroundColor Red
    exit 1
}

Write-Host "Detected: Windows x64" -ForegroundColor Green
Write-Host ""

# Download URL
$DownloadUrl = "https://github.com/$GitHubRepo/releases/download/v$SSLVersion/$BinaryName"

Write-Host "Downloading SSL v$SSLVersion..."
Write-Host "From: $DownloadUrl"
Write-Host ""

# Dry run mode
if ($DryRun) {
    Write-Host "DRY RUN MODE - No files will be downloaded or installed" -ForegroundColor Yellow
    Write-Host "Would download from: $DownloadUrl"
    Write-Host "Would install to: $InstallDir\ssl.exe"
    exit 0
}

# Create install directory
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Download binary
$TmpFile = "$env:TEMP\ssl-installer-$BinaryName"
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TmpFile -UseBasicParsing
}
catch {
    Write-Host "Error downloading SSL: $_" -ForegroundColor Red
    exit 1
}

# Install
Write-Host "Installing to $InstallDir..."
Move-Item -Path $TmpFile -Destination "$InstallDir\ssl.exe" -Force

# Add to PATH if not already present
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
if ($CurrentPath -notlike "*$InstallDir*") {
    Write-Host "Adding SSL to system PATH..."
    $NewPath = "$CurrentPath;$InstallDir"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "Machine")
    Write-Host "  PATH updated (restart terminal to apply)"
}

# Verify
Write-Host ""
Write-Host "✓ Installation complete!" -ForegroundColor Green
Write-Host ""

Write-Host "Verifying installation..."
try {
    & "$InstallDir\ssl.exe" --version
    Write-Host "✓ SSL is working correctly" -ForegroundColor Green
}
catch {
    Write-Host "⚠ SSL installed but verification failed" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Run 'ssl --help' to get started!"
Write-Host "(Restart your terminal if 'ssl' command is not found)"
