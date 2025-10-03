#!/usr/bin/env pwsh
# Git pre-push hook for FerrisScript
# Runs documentation linting before pushing to catch issues early
#
# Installation (OPTIONAL):
#   1. Make this file executable (if on Unix): chmod +x .git/hooks/pre-push
#   2. Copy to .git/hooks/pre-push
#   OR use the install script: .\scripts\install-git-hooks.ps1
#
# To bypass this hook (not recommended):
#   git push --no-verify

$ErrorActionPreference = "Continue"

Write-Host ""
Write-Host "🔍 Running pre-push documentation checks..." -ForegroundColor Cyan
Write-Host ""

# Check if any .md files are being pushed
$mdFilesChanged = git diff --name-only "@{u}.." | Select-String -Pattern "\.md$"

if (-not $mdFilesChanged) {
    Write-Host "✅ No markdown files changed, skipping documentation checks" -ForegroundColor Green
    exit 0
}

Write-Host "📄 Markdown files changed:" -ForegroundColor Yellow
$mdFilesChanged | ForEach-Object { Write-Host "   - $_" -ForegroundColor Yellow }
Write-Host ""

# Check if Node.js is installed
$nodeInstalled = Get-Command node -ErrorAction SilentlyContinue
if (-not $nodeInstalled) {
    Write-Host "⚠️  Node.js not installed - skipping documentation checks" -ForegroundColor Yellow
    Write-Host "   Install Node.js to enable pre-push documentation validation" -ForegroundColor Yellow
    Write-Host "   Download: https://nodejs.org/" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "🚀 Pushing anyway (checks will run in CI)..." -ForegroundColor Yellow
    exit 0
}

# Check if npm packages are installed
if (-not (Test-Path "node_modules")) {
    Write-Host "📦 Installing npm dependencies..." -ForegroundColor Yellow
    npm install --silent
    if ($LASTEXITCODE -ne 0) {
        Write-Host ""
        Write-Host "❌ Failed to install npm dependencies" -ForegroundColor Red
        Write-Host "   Run manually: npm install" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "🚀 Pushing anyway (checks will run in CI)..." -ForegroundColor Yellow
        exit 0
    }
}

# Run documentation linting
Write-Host "🔧 Running markdownlint..." -ForegroundColor Cyan
npx markdownlint '**/*.md' --ignore node_modules --ignore target --dot
$lintExitCode = $LASTEXITCODE

if ($lintExitCode -ne 0) {
    Write-Host ""
    Write-Host "❌ Documentation linting failed!" -ForegroundColor Red
    Write-Host ""
    Write-Host "To fix automatically, run:" -ForegroundColor Yellow
    Write-Host "   npm run docs:fix" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "To bypass this check (not recommended):" -ForegroundColor Yellow
    Write-Host "   git push --no-verify" -ForegroundColor Cyan
    Write-Host ""
    exit 1
}

Write-Host "✅ Documentation checks passed!" -ForegroundColor Green
Write-Host ""
exit 0
