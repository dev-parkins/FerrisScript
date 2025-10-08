#!/usr/bin/env bash
# Git Hooks Uninstaller for FerrisScript

echo "=== FerrisScript Git Hooks Uninstaller ==="
echo ""

HOOKS_DIR=".git/hooks"
PRE_PUSH_HOOK="$HOOKS_DIR/pre-push"
PRE_COMMIT_HOOK="$HOOKS_DIR/pre-commit"

# Check if in a git repository
if [ ! -d ".git" ]; then
    echo "❌ Error: Not in a git repository root"
    echo "   Run this script from the FerrisScript root directory"
    exit 1
fi

# Check if hooks directory exists
if [ ! -d "$HOOKS_DIR" ]; then
    echo "❌ Error: .git/hooks directory not found"
    exit 1
fi

echo "📋 Uninstalling git hooks..."
echo ""

removed=0

# Remove pre-commit hook
if [ -f "$PRE_COMMIT_HOOK" ]; then
    rm "$PRE_COMMIT_HOOK"
    echo "✅ Pre-commit hook removed"
    removed=$((removed + 1))
else
    echo "ℹ️  Pre-commit hook not found (already uninstalled)"
fi

# Remove pre-push hook
if [ -f "$PRE_PUSH_HOOK" ]; then
    rm "$PRE_PUSH_HOOK"
    echo "✅ Pre-push hook removed"
    removed=$((removed + 1))
else
    echo "ℹ️  Pre-push hook not found (already uninstalled)"
fi

echo ""

if [ $removed -gt 0 ]; then
    echo "🎉 Successfully uninstalled $removed hook(s)!"
    echo ""
    echo "Note: You can reinstall hooks anytime by running:"
    echo "  ./scripts/install-git-hooks.sh"
else
    echo "ℹ️  No hooks were installed"
fi
