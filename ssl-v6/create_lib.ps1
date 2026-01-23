# Create Runtime Library
# Creates ssl_runtime.lib from assembled object files

Write-Host "Creating SSL Runtime Library..." -ForegroundColor Cyan
Write-Host ""

# Setup VS environment
$vsPath = "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat"

if (-not (Test-Path $vsPath)) {
    Write-Host "❌ Visual Studio not found at: $vsPath" -ForegroundColor Red
    exit 1
}

# Navigate to runtime directory
cd "$PSScriptRoot\runtime"

Write-Host "Setting up environment and creating library..." -ForegroundColor Yellow

# Run lib command through cmd with vcvars
$command = "`"$vsPath`" && lib /OUT:ssl_runtime.lib memory.obj string.obj list.obj io.obj"

$output = cmd /c $command 2>&1

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ ssl_runtime.lib created successfully!" -ForegroundColor Green
    
    if (Test-Path "ssl_runtime.lib") {
        $size = (Get-Item "ssl_runtime.lib").Length
        Write-Host ""
        Write-Host "Library Details:" -ForegroundColor Cyan
        Write-Host "  File: ssl_runtime.lib"
        Write-Host "  Size: $size bytes"
        Write-Host "  Modules: memory, string, list, io"
        Write-Host ""
        Write-Host "✅ Runtime library ready for linking!" -ForegroundColor Green
    }
}
else {
    Write-Host "❌ Failed to create library" -ForegroundColor Red
    Write-Host "Output: $output" -ForegroundColor Gray
}

cd ..
