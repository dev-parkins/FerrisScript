# FerrisScript Linting (PowerShell)
# Runs cargo clippy with strict warnings

$ErrorActionPreference = "Stop"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "FerrisScript Linting (Clippy)" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

# Run clippy on all workspace crates
Write-Host "Running clippy on workspace..." -ForegroundColor Yellow
cargo clippy --workspace --all-targets --all-features -- -D warnings

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "❌ Linting failed! Fix warnings above." -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host ""
Write-Host "==========================================" -ForegroundColor Green
Write-Host "✅ All linting checks passed!" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
