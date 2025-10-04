# FerrisScript Benchmark Runner (PowerShell)
# Runs performance benchmarks for the compiler

$ErrorActionPreference = "Stop"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "FerrisScript Benchmark Suite" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "⚠️  Note: Benchmarks require 'criterion' in Cargo.toml" -ForegroundColor Yellow
Write-Host ""

Write-Host "Running compiler benchmarks..." -ForegroundColor Yellow
cargo bench --package ferrisscript_compiler

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "❌ Benchmarks failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host ""
Write-Host "==========================================" -ForegroundColor Green
Write-Host "✅ Benchmarks complete!" -ForegroundColor Green
Write-Host "Results saved to: target/criterion/" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
