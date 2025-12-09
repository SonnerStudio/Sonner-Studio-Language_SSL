# SSL v7.0 One-Line Installer for Windows
# Usage: iwr -useb https://raw.githubusercontent.com/SonnerStudio/Sonner-Studio-Language_SSL/main/install.ps1 | iex

Write-Host "Installing SSL v7.0..." -ForegroundColor Cyan

# Download latest release
$version = "v7.0.0"
$url = "https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/$version/ssl-windows-x64.zip"
$tempZip = "$env:TEMP\ssl.zip"
$installPath = "$env:LOCALAPPDATA\SSL"

try {
    Write-Host "Downloading SSL v7.0..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri $url -OutFile $tempZip -UseBasicParsing
    
    Write-Host "Installing to $installPath..." -ForegroundColor Yellow
    
    # Remove old installation
    if (Test-Path $installPath) {
        Remove-Item -Recurse -Force $installPath
    }
    
    # Extract
    Expand-Archive -Path $tempZip -DestinationPath $installPath -Force
    
    # Add to PATH if not already there
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($currentPath -notlike "*$installPath*") {
        Write-Host "Adding to PATH..." -ForegroundColor Yellow
        [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$installPath", "User")
        $env:PATH = "$env:PATH;$installPath"
    }
    
    # Cleanup
    Remove-Item $tempZip -Force
    
    Write-Host "`n✅ SSL v7.0 installed successfully!" -ForegroundColor Green
    Write-Host "`nTest it:" -ForegroundColor Cyan
    Write-Host "  ssl --version" -ForegroundColor White
    Write-Host "`nRestart your terminal or run:" -ForegroundColor Cyan
    Write-Host '  $env:PATH = [System.Environment]::GetEnvironmentVariable("PATH","User")' -ForegroundColor White
    
}
catch {
    Write-Host "❌ Installation failed: $_" -ForegroundColor Red
    exit 1
}
