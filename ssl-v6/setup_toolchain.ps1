# Setup ML64 Environment
# Aktiviert Visual Studio Build Tools für diese Session

$vsPath = "C:\Program Files\Microsoft Visual Studio\2022"

# Try Community, then Professional, then BuildTools
$editions = @("Community", "Professional", "BuildTools", "Enterprise")

foreach ($edition in $editions) {
    $vcvars = "$vsPath\$edition\VC\Auxiliary\Build\vcvars64.bat"
    
    if (Test-Path $vcvars) {
        Write-Host "Found Visual Studio $edition" -ForegroundColor Green
        Write-Host "Setting up environment..." -ForegroundColor Yellow
        
        # Execute vcvars64.bat and import environment variables
        cmd /c "`"$vcvars`" && set" | ForEach-Object {
            if ($_ -match "^(.*?)=(.*)$") {
                Set-Content "env:\$($matches[1])" $matches[2]
            }
        }
        
        Write-Host ""
        Write-Host "✅ Environment ready!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Verifying tools:"
        
        # Verify ml64
        $ml64 = Get-Command ml64 -ErrorAction SilentlyContinue
        if ($ml64) {
            Write-Host "  ✅ ml64: $($ml64.Source)" -ForegroundColor Green
        }
        else {
            Write-Host "  ❌ ml64: Not found" -ForegroundColor Red
        }
        
        # Verify link
        $link = Get-Command link -ErrorAction SilentlyContinue
        if ($link) {
            Write-Host "  ✅ link: $($link.Source)" -ForegroundColor Green
        }
        else {
            Write-Host "  ❌ link: Not found" -ForegroundColor Red
        }
        
        Write-Host ""
        Write-Host "Ready to build runtime!" -ForegroundColor Cyan
        
        return
    }
}

Write-Host "❌ Visual Studio not found" -ForegroundColor Red
Write-Host "Please install Visual Studio Build Tools" -ForegroundColor Yellow
