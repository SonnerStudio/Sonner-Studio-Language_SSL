# SSL v9 Aurora - Bootstrap Installer
# This script downloads the latest SSL v9 binary from GitHub and sets it up for use in your IDE.

$repo = "SonnerStudio/Sonner-Studio-Language_SSL"
$binName = "ssl-v9.exe"
$localPath = Join-Path $PWD $binName

Write-Host "🚀 Initializing SSL v9 Bootstrap Installer..." -ForegroundColor Cyan

# 1. Fetch latest release info from GitHub API
try {
    Write-Host "🔍 Checking for latest release on GitHub..." -ForegroundColor Gray
    $releaseUrl = "https://api.github.com/repos/$repo/releases/latest"
    $release = Invoke-RestMethod -Uri $releaseUrl -Method Get
    $version = $release.tag_name
    Write-Host "✅ Found version: $version" -ForegroundColor Green

    # 2. Find the download URL for the binary
    $asset = $release.assets | Where-Object { $_.name -eq $binName }
    if (-not $asset) {
        throw "Could not find $binName in the latest release assets."
    }
    $downloadUrl = $asset.browser_download_url

    # 3. Download the binary
    Write-Host "📥 Downloading $binName..." -ForegroundColor Gray
    Invoke-WebRequest -Uri $downloadUrl -OutFile $localPath
    Write-Host "✅ Download complete: $localPath" -ForegroundColor Green

    # 4. Final Instructions
    Write-Host "`n🎉 SSL v9 Setup Successful!" -ForegroundColor Cyan
    Write-Host "--------------------------------------------------"
    Write-Host "To use SSL in your IDE:"
    Write-Host "1. Open your terminal in this directory."
    Write-Host "2. Run: .\$binName doctor"
    Write-Host "3. Point your IDE/LSP settings to this EXE path."
    Write-Host "--------------------------------------------------"
}
catch {
    Write-Host "❌ Error during setup: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "Please check your internet connection or GitHub rate limits." -ForegroundColor Yellow
}
