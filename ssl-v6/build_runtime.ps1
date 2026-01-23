# Build SSL Runtime Library
# Requires: ml64 and lib tools in PATH (run setup_toolchain.ps1 first)

Write-Host "==========================================="  -ForegroundColor Cyan
Write-Host " SSL v6.2 - Building Runtime Library"  -ForegroundColor Cyan
Write-Host "==========================================="  -ForegroundColor Cyan
Write-Host ""

# Setup environment first
.$PSScriptRoot\setup_toolchain.ps1

Write-Host ""
Write-Host "[1/3] Assembling runtime modules..." -ForegroundColor Yellow

cd runtime

$modules = @("memory", "string", "list", "io", "convert")
$success = 0
$failed = 0

foreach ($module in $modules) {
    Write-Host "  Assembling $module.asm..." -NoNewline
    
    $result = ml64 /c /Fo "$module.obj" "$module.asm" 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host " ✅" -ForegroundColor Green
        $success++
    }
    else {
        Write-Host " ❌" -ForegroundColor Red
        Write-Host "    Error: $result" -ForegroundColor Red
        $failed++
    }
}

Write-Host ""
Write-Host "[2/3] Creating runtime library..." -ForegroundColor Yellow

if ($failed -eq 0) {
    $objFiles = $modules | ForEach-Object { "$_.obj" }
    $libCmd = "lib /OUT:ssl_runtime.lib " + ($objFiles -join " ")
    
    Write-Host "  Running: $libCmd" -ForegroundColor Gray
    
    Invoke-Expression $libCmd
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✅ ssl_runtime.lib created" -ForegroundColor Green
    }
    else {
        Write-Host "  ❌ Failed to create library" -ForegroundColor Red
        $failed++
    }
}
else {
    Write-Host "  ⏭ Skipped (assembly errors)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "[3/3] Summary" -ForegroundColor Yellow
Write-Host "  Assembled: $success/$($modules.Count)" -ForegroundColor Green
Write-Host "  Failed: $failed" -ForegroundColor Red

if (Test-Path "ssl_runtime.lib") {
    $size = (Get-Item "ssl_runtime.lib").Length
    Write-Host "  Library size: $size bytes" -ForegroundColor Cyan
}

cd ..

Write-Host ""
if ($failed -eq 0) {
    Write-Host "==========================================="  -ForegroundColor Green
    Write-Host " ✅ Runtime Library Built Successfully!"  -ForegroundColor Green
    Write-Host "==========================================="  -ForegroundColor Green
}
else {
    Write-Host "==========================================="  -ForegroundColor Red
    Write-Host " ⚠️ Build completed with errors"  -ForegroundColor Red
    Write-Host "==========================================="  -ForegroundColor Red
}
