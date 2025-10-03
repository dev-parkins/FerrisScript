#!/usr/bin/env pwsh
# Run test coverage analysis with cargo-tarpaulin
# Usage: ./scripts/coverage.ps1

Write-Host "🔍 Running test coverage analysis..." -ForegroundColor Cyan

# Check if tarpaulin is installed
if (!(Get-Command cargo-tarpaulin -ErrorAction SilentlyContinue)) {
    Write-Host "❌ cargo-tarpaulin not found. Installing..." -ForegroundColor Yellow
    cargo install cargo-tarpaulin
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to install cargo-tarpaulin" -ForegroundColor Red
        exit 1
    }
}

# Create coverage output directory
New-Item -ItemType Directory -Force -Path "target/coverage" | Out-Null

# Run coverage
Write-Host "📊 Analyzing coverage across workspace..." -ForegroundColor Cyan
cargo tarpaulin --workspace --out Html --out Lcov --out Stdout --output-dir target/coverage

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ Coverage analysis complete!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📄 Reports generated:" -ForegroundColor Cyan
    Write-Host "  - HTML: target/coverage/tarpaulin-report.html" -ForegroundColor Gray
    Write-Host "  - LCOV: target/coverage/lcov.info" -ForegroundColor Gray
    Write-Host ""
    Write-Host "🌐 Open HTML report:" -ForegroundColor Cyan
    Write-Host "  Invoke-Item target/coverage/tarpaulin-report.html" -ForegroundColor Gray
} else {
    Write-Host ""
    Write-Host "❌ Coverage analysis failed" -ForegroundColor Red
    exit 1
}
