# Clean previous builds
Remove-Item -Recurse -Force dist, build, release -ErrorAction SilentlyContinue

Write-Host "Building SSL v7.0 for Windows x64..." -ForegroundColor Cyan

# Build Windows binary
python -m PyInstaller --onefile --name ssl --icon=assets\ssl-v7-logo.png ssl-v7\ssl.py

if (-not $?) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

# Create release directory
New-Item -ItemType Directory -Force release\windows-x64 | Out-Null

# Copy files
Copy-Item dist\ssl.exe release\windows-x64\
Copy-Item ssl-v7\README.txt release\windows-x64\
Copy-Item LICENSE release\windows-x64\ -ErrorAction SilentlyContinue

# If no LICENSE, create one
if (-not (Test-Path release\windows-x64\LICENSE)) {
    @"
Apache License 2.0

Copyright (c) 2025 SonnerStudio

Licensed under the Apache License, Version 2.0
"@ | Out-File release\windows-x64\LICENSE -Encoding UTF8
}

# Create ZIP
Compress-Archive -Path release\windows-x64\* -DestinationPath release\ssl-windows-x64.zip -Force

# Calculate checksum
$hash = (CertUtil -hashfile release\ssl-windows-x64.zip SHA256 | Select-String -Pattern "^[0-9a-fA-F]{64}$").Matches.Value
$hash | Out-File release\ssl-windows-x64.zip.sha256 -Encoding ASCII

# Display results
Write-Host "`n✅ Build completed successfully!" -ForegroundColor Green
Write-Host "`nPackage: release\ssl-windows-x64.zip" -ForegroundColor Yellow
Write-Host "Size: $([math]::Round((Get-Item release\ssl-windows-x64.zip).Length / 1MB, 2)) MB" -ForegroundColor Yellow
Write-Host "SHA256: $hash" -ForegroundColor Yellow

# Test the executable
Write-Host "`nTesting executable..." -ForegroundColor Cyan
& dist\ssl.exe --version

Write-Host "`n🎉 Ready for release!" -ForegroundColor Green
