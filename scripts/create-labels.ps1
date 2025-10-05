# Create GitHub Labels for FerrisScript
# Part of v0.0.2 Release - Phase 5A
#
# Prerequisites:
#   - GitHub CLI installed: https://cli.github.com/
#   - Authenticated: gh auth login
#
# Usage:
#   .\scripts\create-labels.ps1
#
# This script creates 20 labels across 5 categories

$ErrorActionPreference = "Stop"

Write-Host "🏷️  Creating GitHub Labels for FerrisScript..." -ForegroundColor Cyan
Write-Host ""

# Check if gh CLI is available
if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Error: GitHub CLI (gh) is not installed" -ForegroundColor Red
    Write-Host "   Install from: https://cli.github.com/" -ForegroundColor Yellow
    exit 1
}

# Check if authenticated
try {
    gh auth status 2>&1 | Out-Null
} catch {
    Write-Host "❌ Error: Not authenticated with GitHub CLI" -ForegroundColor Red
    Write-Host "   Run: gh auth login" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ GitHub CLI is authenticated" -ForegroundColor Green
Write-Host ""

# Function to create label (idempotent - won't fail if exists)
function Create-Label {
    param(
        [string]$Name,
        [string]$Description,
        [string]$Color
    )
    
    try {
        gh label create $Name --description $Description --color $Color 2>&1 | Out-Null
        Write-Host "  ✓ Created: $Name" -ForegroundColor Green
    } catch {
        Write-Host "  ℹ Already exists: $Name" -ForegroundColor Gray
    }
}

# Priority Labels
Write-Host "📌 Creating Priority Labels..." -ForegroundColor Yellow
Create-Label "P0-Critical" "Critical bugs or blockers requiring immediate attention" "d73a4a"
Create-Label "P1-High" "High priority tasks that should be addressed soon" "ff6600"
Create-Label "P2-Medium" "Medium priority tasks for regular workflow" "fbca04"
Create-Label "P3-Low" "Low priority tasks or nice-to-have improvements" "0e8a16"
Write-Host ""

# Type Labels
Write-Host "🏷️  Creating Type Labels..." -ForegroundColor Yellow
Create-Label "bug" "Something isn't working correctly" "d73a4a"
Create-Label "feature" "New feature or functionality request" "a2eeef"
Create-Label "documentation" "Documentation improvements or additions" "0075ca"
Create-Label "enhancement" "Improvement to existing functionality" "84b6eb"
Create-Label "question" "Questions or clarifications needed" "d876e3"
Create-Label "discussion" "General discussion topics" "cc317c"
Write-Host ""

# Status Labels
Write-Host "📊 Creating Status Labels..." -ForegroundColor Yellow
Create-Label "needs-triage" "New issue awaiting initial review and prioritization" "e4e669"
Create-Label "in-progress" "Work is actively being done on this issue" "fbca04"
Create-Label "blocked" "Blocked by external dependencies or decisions" "b60205"
Create-Label "wontfix" "Issue will not be addressed (with explanation)" "ffffff"
Write-Host ""

# Difficulty Labels
Write-Host "🎯 Creating Difficulty Labels..." -ForegroundColor Yellow
Create-Label "good-first-issue" "Good for newcomers to the project" "7057ff"
Create-Label "intermediate" "Requires moderate knowledge of codebase" "008672"
Create-Label "advanced" "Requires deep understanding of architecture" "5319e7"
Write-Host ""

# Component Labels
Write-Host "🔧 Creating Component Labels..." -ForegroundColor Yellow
Create-Label "compiler" "Related to compiler crate (lexer, parser, type checker)" "1d76db"
Create-Label "runtime" "Related to runtime crate (execution environment)" "0e8a16"
Create-Label "godot-bind" "Related to Godot GDExtension bindings" "fbca04"
Create-Label "docs" "Related to documentation (not code)" "0075ca"
Create-Label "ci" "Related to CI/CD, GitHub Actions, workflows" "ededed"
Write-Host ""

Write-Host "✅ Label creation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "📖 Documentation: docs/GITHUB_LABELS.md" -ForegroundColor Cyan
Write-Host "🔗 View labels: https://github.com/dev-parkins/FerrisScript/labels" -ForegroundColor Cyan
