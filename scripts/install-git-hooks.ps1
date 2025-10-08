# Git Hooks Installer for FerrisScript

Write-Host "=== FerrisScript Git Hooks Installer ===" -ForegroundColor Cyan
Write-Host ""

$hooksDir = ".git/hooks"
$prePushSource = "scripts/pre-push.ps1"
$prePushDest = "$hooksDir/pre-push"
$preCommitTemplate = @"
#!/usr/bin/env bash
# FerrisScript pre-commit hook
# Runs format check, linting, and quick tests before allowing commit

set -e

echo "🔍 Running pre-commit checks..."
echo ""

# 1. Format check
echo "Checking code formatting..."
cargo fmt --check
if [ \`$? -ne 0 ]; then
    echo "❌ Code formatting check failed!"
    echo "Run 'cargo fmt' or './scripts/format.sh' to fix formatting."
    exit 1
fi
echo "✅ Formatting OK"
echo ""

# 2. Clippy linting
echo "Running clippy linting..."
cargo clippy --workspace --all-targets -- -D warnings
if [ \`$? -ne 0 ]; then
    echo "❌ Linting failed!"
    echo "Fix clippy warnings above or run './scripts/lint.sh' for details."
    exit 1
fi
echo "✅ Linting OK"
echo ""

# 3. Quick tests (skip slow integration tests)
echo "Running quick tests..."
cargo test --workspace --lib
if [ \`$? -ne 0 ]; then
    echo "❌ Tests failed!"
    echo "Fix failing tests or run './scripts/test.sh' for full output."
    exit 1
fi
echo "✅ Tests OK"
echo ""

echo "✅ All pre-commit checks passed! Proceeding with commit..."
"@
$preCommitDest = "$hooksDir/pre-commit"

# Check if in a git repository
if (-not (Test-Path ".git")) {
    Write-Host "❌ Error: Not in a git repository root" -ForegroundColor Red
    Write-Host "   Run this script from the FerrisScript root directory" -ForegroundColor Yellow
    exit 1
}

# Check if hooks directory exists
if (-not (Test-Path $hooksDir)) {
    Write-Host "❌ Error: .git/hooks directory not found" -ForegroundColor Red
    exit 1
}

# Check if source hook exists
if (-not (Test-Path $prePushSource)) {
    Write-Host "❌ Error: $prePushSource not found" -ForegroundColor Red
    exit 1
}

Write-Host "📋 Installing git hooks..." -ForegroundColor Cyan
Write-Host ""

# Install pre-push hook
Write-Host "Installing pre-push hook..." -ForegroundColor Yellow

# Create hook wrapper for PowerShell on Windows
$hookContent = @"
#!/bin/sh
# Git pre-push hook for FerrisScript
# Calls the PowerShell script for actual implementation

if command -v pwsh >/dev/null 2>&1; then
    pwsh -File "$prePushSource"
elif command -v powershell >/dev/null 2>&1; then
    powershell -File "$prePushSource"
else
    echo "⚠️  PowerShell not found - skipping documentation checks"
    exit 0
fi
"@

Set-Content -Path $prePushDest -Value $hookContent -Encoding UTF8

# Make executable on Unix-like systems (Git Bash on Windows handles this)
if ($IsLinux -or $IsMacOS) {
    chmod +x $prePushDest
}

Write-Host "✅ Pre-push hook installed" -ForegroundColor Green
Write-Host ""

# Install pre-commit hook
Write-Host "Installing pre-commit hook..." -ForegroundColor Yellow
Set-Content -Path $preCommitDest -Value $preCommitTemplate -Encoding UTF8

# Make executable on Unix-like systems
if ($IsLinux -or $IsMacOS) {
    chmod +x $preCommitDest
}

Write-Host "✅ Pre-commit hook installed" -ForegroundColor Green
Write-Host ""

Write-Host "=== Installation Complete ===" -ForegroundColor Green
Write-Host ""
Write-Host "The following hooks are now active:" -ForegroundColor Cyan
Write-Host "  • pre-commit: Validates code format, linting, and tests before commit" -ForegroundColor White
Write-Host "  • pre-push: Validates documentation before pushing" -ForegroundColor White
Write-Host ""
Write-Host "What this means:" -ForegroundColor Yellow
Write-Host "  ✅ Code quality checks run automatically before every commit" -ForegroundColor White
Write-Host "  ✅ Markdown linting runs automatically before every push" -ForegroundColor White
Write-Host "  ✅ Catches issues before CI runs" -ForegroundColor White
Write-Host "  ✅ Can be bypassed with: git commit/push --no-verify" -ForegroundColor White
Write-Host ""
Write-Host "To uninstall:" -ForegroundColor Yellow
Write-Host "  Remove-Item .git/hooks/pre-commit" -ForegroundColor Cyan
Write-Host "  Remove-Item .git/hooks/pre-push" -ForegroundColor Cyan
Write-Host ""
