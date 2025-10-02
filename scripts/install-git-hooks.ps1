# Git Hooks Installer for FerrisScript

Write-Host "=== FerrisScript Git Hooks Installer ===" -ForegroundColor Cyan
Write-Host ""

$hooksDir = ".git/hooks"
$prePushSource = "scripts/pre-push.ps1"
$prePushDest = "$hooksDir/pre-push"

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

Write-Host "=== Installation Complete ===" -ForegroundColor Green
Write-Host ""
Write-Host "The following hook is now active:" -ForegroundColor Cyan
Write-Host "  • pre-push: Validates documentation before pushing" -ForegroundColor White
Write-Host ""
Write-Host "What this means:" -ForegroundColor Yellow
Write-Host "  ✅ Markdown linting runs automatically before every push" -ForegroundColor White
Write-Host "  ✅ Catches formatting issues before CI runs" -ForegroundColor White
Write-Host "  ✅ Only runs when .md files are changed" -ForegroundColor White
Write-Host "  ✅ Can be bypassed with: git push --no-verify" -ForegroundColor White
Write-Host ""
Write-Host "To uninstall:" -ForegroundColor Yellow
Write-Host "  Remove-Item .git/hooks/pre-push" -ForegroundColor Cyan
Write-Host ""
