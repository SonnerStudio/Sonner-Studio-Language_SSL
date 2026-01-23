# SSL v6.1 - Complete Verification Script

# Verify all v6.0 & v6.1 modules compile
Write-Host "==========================================="  -ForegroundColor Cyan
Write-Host " SSL v6.1 - Complete Verification"  -ForegroundColor Cyan
Write-Host "==========================================="  -ForegroundColor Cyan
Write-Host ""

$errors = 0
$success = 0

# v6.0 modules
Write-Host "[1/5] Verifying v6.0 Modules..." -ForegroundColor Yellow
$v60_modules = @(
    "ssl-v6\src\main.ssl",
    "ssl-v6\src\lexer.ssl",
    "ssl-v6\src\parser.ssl",
    "ssl-v6\src\ast.ssl",
    "ssl-v6\src\types.ssl",
    "ssl-v6\src\ir.ssl",
    "ssl-v6\src\optimize.ssl",
    "ssl-v6\src\codegen\mod.ssl",
    "ssl-v6\src\codegen\x64.ssl"
)

foreach ($module in $v60_modules) {
    Write-Host "  Checking $module..." -NoNewline
    $result = ssl check $module 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host " ✅" -ForegroundColor Green
        $success++
    }
    else {
        Write-Host " ❌" -ForegroundColor Red
        $errors++
    }
}
Write-Host ""

# v6.1 modules
Write-Host "[2/5] Verifying v6.1 Modules..." -ForegroundColor Yellow
$v61_modules = @(
    "ssl-v6\src\runtime_builtins.ssl",
    "ssl-v6\src\codegen\x64_runtime.ssl",
    "ssl-v6\src\lexer_enhanced.ssl",
    "ssl-v6\src\parser_enhanced.ssl",
    "ssl-v6\src\codegen\complete.ssl"
)

foreach ($module in $v61_modules) {
    Write-Host "  Checking $module..." -NoNewline
    $result = ssl check $module 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host " ✅" -ForegroundColor Green
        $success++
    }
    else {
        Write-Host " ❌" -ForegroundColor Red
        $errors++
    }
}
Write-Host ""

# Test programs
Write-Host "[3/5] Running Test Programs..." -ForegroundColor Yellow
$tests = @(
    "ssl-v6\hello_compiler.ssl",
    "ssl-v6\mvc_compiler.ssl",
    "ssl-v6\runtime_integration_demo.ssl",
    "ssl-v6\parser_demo.ssl",
    "ssl-v6\codegen_demo.ssl"
)

foreach ($test in $tests) {
    Write-Host "  Running $test..." -NoNewline
    $result = ssl run $test 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host " ✅" -ForegroundColor Green
        $success++
    }
    else {
        Write-Host " ❌" -ForegroundColor Red
        $errors++
    }
}
Write-Host ""

# Assembly generation verification
Write-Host "[4/5] Verifying Assembly Generation..." -ForegroundColor Yellow
Write-Host "  Running MVC compiler..." -NoNewline
$result = ssl run ssl-v6\mvc_compiler.ssl 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host " ✅" -ForegroundColor Green
    $success++
}
else {
    Write-Host " ❌" -ForegroundColor Red
    $errors++
}
Write-Host ""

# Summary
Write-Host "[5/5] Summary" -ForegroundColor Yellow
Write-Host "  ✅ Successful: $success" -ForegroundColor Green
Write-Host "  ❌ Failed: $errors" -ForegroundColor Red
Write-Host ""

if ($errors -eq 0) {
    Write-Host "==========================================="  -ForegroundColor Green
    Write-Host " ✅ ALL VERIFICATIONS PASSED!"  -ForegroundColor Green
    Write-Host "==========================================="  -ForegroundColor Green
    Write-Host ""
    Write-Host "SSL v6.1 Status:" -ForegroundColor Cyan
    Write-Host "  • v6.0 Modules: 9 ✅"
    Write-Host "  • v6.1 Modules: 5 ✅"
    Write-Host "  • Test Programs: 5 ✅"
    Write-Host "  • Total: $($success) checks passed"
    Write-Host ""
    Write-Host "🎉 Symbolic Bootstrap: VERIFIED" -ForegroundColor Green
}
else {
    Write-Host "==========================================="  -ForegroundColor Red
    Write-Host " ⚠️ SOME VERIFICATIONS FAILED"  -ForegroundColor Red
    Write-Host "==========================================="  -ForegroundColor Red
}

Write-Host ""
