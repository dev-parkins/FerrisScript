#!/usr/bin/env bash
# Git Hooks Installer for FerrisScript

echo "=== FerrisScript Git Hooks Installer ==="
echo ""

HOOKS_DIR=".git/hooks"
PRE_PUSH_SOURCE="scripts/pre-push.sh"
PRE_PUSH_DEST="$HOOKS_DIR/pre-push"

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

# Check if source hook exists
if [ ! -f "$PRE_PUSH_SOURCE" ]; then
    echo "❌ Error: $PRE_PUSH_SOURCE not found"
    exit 1
fi

echo "📋 Installing git hooks..."
echo ""

# Install pre-push hook
echo "Installing pre-push hook..."

# Copy the bash script as the hook
cp "$PRE_PUSH_SOURCE" "$PRE_PUSH_DEST"

# Make executable
chmod +x "$PRE_PUSH_DEST"

echo "✅ Pre-push hook installed"
echo ""

echo "=== Installation Complete ==="
echo ""
echo "The following hook is now active:"
echo "  • pre-push: Validates documentation before pushing"
echo ""
echo "What this means:"
echo "  ✅ Markdown linting runs automatically before every push"
echo "  ✅ Catches formatting issues before CI runs"
echo "  ✅ Only runs when .md files are changed"
echo "  ✅ Can be bypassed with: git push --no-verify"
echo ""
echo "To uninstall:"
echo "  rm .git/hooks/pre-push"
echo ""
