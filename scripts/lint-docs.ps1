# Documentation Linting Script for FerrisScript
# Usage: .\scripts\lint-docs.ps1 [--fix]

param(
    [switch]$Fix = $false
)

Write-Host "=== FerrisScript Documentation Linting ===" -ForegroundColor Cyan
Write-Host ""

# Check if Node.js is installed
$nodeInstalled = Get-Command node -ErrorAction SilentlyContinue
if (-not $nodeInstalled) {
    Write-Host "❌ Node.js is not installed. Please install Node.js to run documentation linting." -ForegroundColor Red
    Write-Host "   Download from: https://nodejs.org/" -ForegroundColor Yellow
    exit 1
}

# Check if npm packages are installed
if (-not (Test-Path "node_modules")) {
    Write-Host "📦 Installing npm dependencies..." -ForegroundColor Yellow
    npm install
    Write-Host ""
}

# Run markdownlint
Write-Host "🔍 Step 1/2: Running markdownlint..." -ForegroundColor Cyan
if ($Fix) {
    Write-Host "   Mode: Fix (will auto-fix issues)" -ForegroundColor Yellow
    npm run docs:fix
} else {
    Write-Host "   Mode: Check only" -ForegroundColor Yellow
    npm run docs:lint
}

$markdownlintExitCode = $LASTEXITCODE
Write-Host ""

# Run markdown-link-check
Write-Host "🔗 Step 2/2: Running markdown-link-check..." -ForegroundColor Cyan
npm run docs:links
$linkCheckExitCode = $LASTEXITCODE
Write-Host ""

# Summary
Write-Host "=== Summary ===" -ForegroundColor Cyan
if ($markdownlintExitCode -eq 0 -and $linkCheckExitCode -eq 0) {
    Write-Host "✅ All documentation checks passed!" -ForegroundColor Green
    exit 0
} else {
    if ($markdownlintExitCode -ne 0) {
        Write-Host "❌ Markdownlint found issues" -ForegroundColor Red
        if (-not $Fix) {
            Write-Host "   Run with --fix to auto-fix: .\scripts\lint-docs.ps1 --fix" -ForegroundColor Yellow
        }
    }
    if ($linkCheckExitCode -ne 0) {
        Write-Host "❌ Broken links found" -ForegroundColor Red
    }
    exit 1
}
