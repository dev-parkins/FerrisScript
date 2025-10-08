# Git Hooks Uninstaller for FerrisScript

Write-Host "=== FerrisScript Git Hooks Uninstaller ===" -ForegroundColor Cyan
Write-Host ""

$hooksDir = ".git/hooks"
$prePushHook = "$hooksDir/pre-push"
$preCommitHook = "$hooksDir/pre-commit"

# Check if in a git repository
if (-not (Test-Path ".git")) {
    Write-Host "❌ Error: Not in a git repository root" -ForegroundColor Red
    Write-Host "   Run this script from the FerrisScript root directory"
    exit 1
}

# Check if hooks directory exists
if (-not (Test-Path $hooksDir)) {
    Write-Host "❌ Error: .git/hooks directory not found" -ForegroundColor Red
    exit 1
}

Write-Host "📋 Uninstalling git hooks..." -ForegroundColor Yellow
Write-Host ""

$removed = 0

# Remove pre-commit hook
if (Test-Path $preCommitHook) {
    Remove-Item $preCommitHook -Force
    Write-Host "✅ Pre-commit hook removed" -ForegroundColor Green
    $removed++
} else {
    Write-Host "ℹ️  Pre-commit hook not found (already uninstalled)" -ForegroundColor Gray
}

# Remove pre-push hook
if (Test-Path $prePushHook) {
    Remove-Item $prePushHook -Force
    Write-Host "✅ Pre-push hook removed" -ForegroundColor Green
    $removed++
} else {
    Write-Host "ℹ️  Pre-push hook not found (already uninstalled)" -ForegroundColor Gray
}

Write-Host ""

if ($removed -gt 0) {
    Write-Host "🎉 Successfully uninstalled $removed hook(s)!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Note: You can reinstall hooks anytime by running:" -ForegroundColor Cyan
    Write-Host "  .\scripts\install-git-hooks.ps1"
} else {
    Write-Host "ℹ️  No hooks were installed" -ForegroundColor Gray
}
