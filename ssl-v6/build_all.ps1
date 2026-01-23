# Assemble and Create Runtime Library - All-in-One
# This script does everything in one go with proper paths

Write-Host "==========================================="  -ForegroundColor Cyan
Write-Host " SSL Runtime - Complete Build"  -ForegroundColor Cyan
Write-Host "==========================================="  -ForegroundColor Cyan
Write-Host ""

$vsPath = "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat"

if (-not (Test-Path $vsPath)) {
    Write-Host "❌ Visual Studio not found" -ForegroundColor Red
    exit 1
}

$runtimeDir = Join-Path $PSScriptRoot "runtime"
cd $runtimeDir

Write-Host "[1/2] Assembling modules..." -ForegroundColor Yellow

$modules = @("memory", "string", "list", "io")
$assembled = @()

foreach ($module in $modules) {
    Write-Host "  $module.asm..." -NoNewline
    
    if (Test-Path "$module.asm") {
        $cmd = "`"$vsPath`" >nul && ml64 /c /Fo $module.obj $module.asm"
        $result = cmd /c $cmd 2>&1
        
        if (Test-Path "$module.obj") {
            Write-Host " ✅" -ForegroundColor Green
            $assembled += "$module.obj"
        }
        else {
            Write-Host " ❌" -ForegroundColor Red
        }
    }
    else {
        Write-Host " ⏭ (not found)" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "[2/2] Creating library..." -ForegroundColor Yellow

if ($assembled.Count -gt 0) {
    $objList = $assembled -join " "
    $cmd = "`"$vsPath`" >nul && lib /OUT:ssl_runtime.lib $objList"
    
    Write-Host "  lib /OUT:ssl_runtime.lib $objList" -ForegroundColor Gray
    
    $result = cmd /c $cmd 2>&1
    
    if (Test-Path "ssl_runtime.lib") {
        $size = (Get-Item "ssl_runtime.lib").Length
        Write-Host ""
        Write-Host "✅ ssl_runtime.lib created!" -ForegroundColor Green
        Write-Host "   Size: $size bytes" -ForegroundColor Cyan
        Write-Host "   Modules: $($assembled.Count)" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "==========================================="  -ForegroundColor Green
        Write-Host " Runtime Library Ready!"  -ForegroundColor Green
        Write-Host "==========================================="  -ForegroundColor Green
    }
    else {
        Write-Host "❌ Failed to create library" -ForegroundColor Red
        Write-Host "Output: $result" -ForegroundColor Gray
    }
}
else {
    Write-Host "❌ No modules assembled" -ForegroundColor Red
}

cd $PSScriptRoot
