#!/usr/bin/env bash
# Create GitHub Labels for FerrisScript
# Part of v0.0.2 Release - Phase 5A
#
# Prerequisites:
#   - GitHub CLI installed: https://cli.github.com/
#   - Authenticated: gh auth login
#
# Usage:
#   ./scripts/create-labels.sh
#
# This script creates 20 labels across 5 categories

set -euo pipefail

echo "🏷️  Creating GitHub Labels for FerrisScript..."
echo ""

# Check if gh CLI is available
if ! command -v gh &> /dev/null; then
    echo "❌ Error: GitHub CLI (gh) is not installed"
    echo "   Install from: https://cli.github.com/"
    exit 1
fi

# Check if authenticated
if ! gh auth status &> /dev/null; then
    echo "❌ Error: Not authenticated with GitHub CLI"
    echo "   Run: gh auth login"
    exit 1
fi

echo "✅ GitHub CLI is authenticated"
echo ""

# Function to create label (idempotent - won't fail if exists)
create_label() {
    local name="$1"
    local description="$2"
    local color="$3"
    
    if gh label create "$name" --description "$description" --color "$color" 2>/dev/null; then
        echo "  ✓ Created: $name"
    else
        echo "  ℹ Already exists: $name"
    fi
}

# Priority Labels
echo "📌 Creating Priority Labels..."
create_label "P0-Critical" "Critical bugs or blockers requiring immediate attention" "d73a4a"
create_label "P1-High" "High priority tasks that should be addressed soon" "ff6600"
create_label "P2-Medium" "Medium priority tasks for regular workflow" "fbca04"
create_label "P3-Low" "Low priority tasks or nice-to-have improvements" "0e8a16"
echo ""

# Type Labels
echo "🏷️  Creating Type Labels..."
create_label "bug" "Something isn't working correctly" "d73a4a"
create_label "feature" "New feature or functionality request" "a2eeef"
create_label "documentation" "Documentation improvements or additions" "0075ca"
create_label "enhancement" "Improvement to existing functionality" "84b6eb"
create_label "question" "Questions or clarifications needed" "d876e3"
create_label "discussion" "General discussion topics" "cc317c"
echo ""

# Status Labels
echo "📊 Creating Status Labels..."
create_label "needs-triage" "New issue awaiting initial review and prioritization" "e4e669"
create_label "in-progress" "Work is actively being done on this issue" "fbca04"
create_label "blocked" "Blocked by external dependencies or decisions" "b60205"
create_label "wontfix" "Issue will not be addressed (with explanation)" "ffffff"
echo ""

# Difficulty Labels
echo "🎯 Creating Difficulty Labels..."
create_label "good-first-issue" "Good for newcomers to the project" "7057ff"
create_label "intermediate" "Requires moderate knowledge of codebase" "008672"
create_label "advanced" "Requires deep understanding of architecture" "5319e7"
echo ""

# Component Labels
echo "🔧 Creating Component Labels..."
create_label "compiler" "Related to compiler crate (lexer, parser, type checker)" "1d76db"
create_label "runtime" "Related to runtime crate (execution environment)" "0e8a16"
create_label "godot-bind" "Related to Godot GDExtension bindings" "fbca04"
create_label "docs" "Related to documentation (not code)" "0075ca"
create_label "ci" "Related to CI/CD, GitHub Actions, workflows" "ededed"
echo ""

echo "✅ Label creation complete!"
echo ""
echo "📖 Documentation: docs/GITHUB_LABELS.md"
echo "🔗 View labels: https://github.com/dev-parkins/FerrisScript/labels"
