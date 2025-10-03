#!/usr/bin/env pwsh
# Run test coverage analysis with cargo-llvm-cov
# Usage: ./scripts/coverage.ps1
#
# Note: Switched from tarpaulin to llvm-cov due to Windows file locking issues
# See docs/COVERAGE_SETUP_NOTES.md for details

Write-Host "🔍 Running test coverage analysis..." -ForegroundColor Cyan

# Check if llvm-cov is installed
if (!(Get-Command cargo-llvm-cov -ErrorAction SilentlyContinue)) {
    Write-Host "❌ cargo-llvm-cov not found. Installing..." -ForegroundColor Yellow
    
    # Install llvm-tools-preview
    Write-Host "📦 Installing llvm-tools-preview component..." -ForegroundColor Cyan
    rustup component add llvm-tools-preview
    
    # Install cargo-llvm-cov
    Write-Host "📦 Installing cargo-llvm-cov..." -ForegroundColor Cyan
    cargo install cargo-llvm-cov
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to install cargo-llvm-cov" -ForegroundColor Red
        exit 1
    }
}

# Create coverage output directory
New-Item -ItemType Directory -Force -Path "target/coverage" | Out-Null

# Run coverage
Write-Host "📊 Analyzing coverage across workspace..." -ForegroundColor Cyan
cargo llvm-cov --workspace --html --output-dir target/coverage
cargo llvm-cov --workspace --lcov --output-path target/coverage/lcov.info

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ Coverage analysis complete!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📄 Reports generated:" -ForegroundColor Cyan
    Write-Host "  - HTML: target/coverage/html/index.html" -ForegroundColor Gray
    Write-Host "  - LCOV: target/coverage/lcov.info" -ForegroundColor Gray
    Write-Host ""
    Write-Host "🌐 Open HTML report:" -ForegroundColor Cyan
    Write-Host "  Invoke-Item target/coverage/html/index.html" -ForegroundColor Gray
} else {
    Write-Host ""
    Write-Host "❌ Coverage analysis failed" -ForegroundColor Red
    exit 1
}
