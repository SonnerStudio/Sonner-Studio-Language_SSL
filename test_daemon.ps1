#!/usr/bin/env pwsh
# Test script for ssld AI Daemon

Write-Host "Testing ssld AI Daemon..." -ForegroundColor Green

# Start ssld in background
$ssld = Start-Process -FilePath "target/debug/ssld.exe" -NoNewWindow -PassThru -RedirectStandardInput "daemon_input.txt" -RedirectStandardOutput "daemon_output.txt"

Start-Sleep -Seconds 2

# Test 1: Ping
Write-Host "`nTest 1: Ping" -ForegroundColor Yellow
$pingRequest = @{
    jsonrpc = "2.0"
    method = "ping"
    params = @{}
    id = 1
} | ConvertTo-Json -Compress

Set-Content -Path "daemon_input.txt" -Value $pingRequest
Start-Sleep -Seconds 1

# Test 2: Explain Error
Write-Host "`nTest 2: Explain Error" -ForegroundColor Yellow
$explainRequest = @{
    jsonrpc = "2.0"
    method = "explain_error"
    params = @{
        error = "Expected LBrace, got Assign"
    }
    id = 2
} | ConvertTo-Json -Compress

Add-Content -Path "daemon_input.txt" -Value $explainRequest
Start-Sleep -Seconds 3

# Show output
Write-Host "`nDaemon Output:" -ForegroundColor Cyan
Get-Content "daemon_output.txt"

# Cleanup
Stop-Process -Id $ssld.Id -Force
Remove-Item "daemon_input.txt", "daemon_output.txt" -ErrorAction SilentlyContinue
