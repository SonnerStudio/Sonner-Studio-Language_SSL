# SSL v6.2 - Simple Phase 2 Test
# Assemble the generated test.asm

Write-Host "SSL v6.2 - Phase 2: Assemble Test" -ForegroundColor Cyan
Write-Host ""

$testAsm = "$PSScriptRoot\test.asm"

if (-not (Test-Path $testAsm)) {
    Write-Host "ERROR: test.asm not found" -ForegroundColor Red
    exit 1
}

Write-Host "Assembling test.asm..." -ForegroundColor Yellow

# Try to assemble
try {
    $output = ml64 /c /Fo"$PSScriptRoot\test.obj" $testAsm 2>&1
    
    if (Test-Path "$PSScriptRoot\test.obj") {
        Write-Host "✓ Assembly successful" -ForegroundColor Green
        Write-Host ""
        Write-Host "test.obj created!" -ForegroundColor Cyan
    }
    else {
        Write-Host "Assembly failed" -ForegroundColor Red
        Write-Host $output
    }
}
catch {
    Write-Host "ml64 not found - please run setup_toolchain.ps1 first" -ForegroundColor Yellow
}
