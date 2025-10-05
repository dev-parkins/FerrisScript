# FerrisScript Code Formatter (PowerShell)
# Formats all Rust code in the workspace

$ErrorActionPreference = "Stop"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "FerrisScript Code Formatter" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "Formatting all Rust code..." -ForegroundColor Yellow
cargo fmt --all

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "❌ Formatting failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host ""
Write-Host "==========================================" -ForegroundColor Green
Write-Host "✅ Code formatted successfully!" -ForegroundColor Green
Write-Host "Tip: Run 'cargo fmt -- --check' to verify without modifying files" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Green
