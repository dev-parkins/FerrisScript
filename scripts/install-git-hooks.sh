#!/usr/bin/env bash
# Git Hooks Installer for FerrisScript

echo "=== FerrisScript Git Hooks Installer ==="
echo ""

HOOKS_DIR=".git/hooks"
PRE_PUSH_SOURCE="scripts/pre-push.sh"
PRE_PUSH_DEST="$HOOKS_DIR/pre-push"
PRE_COMMIT_DEST="$HOOKS_DIR/pre-commit"

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

# Install pre-commit hook
echo "Installing pre-commit hook..."

cat > "$PRE_COMMIT_DEST" << 'EOF'
#!/usr/bin/env bash
# FerrisScript pre-commit hook
# Runs format check, linting, and quick tests before allowing commit

set -e

echo "🔍 Running pre-commit checks..."
echo ""

# 1. Format check
echo "Checking code formatting..."
cargo fmt --check
if [ $? -ne 0 ]; then
    echo "❌ Code formatting check failed!"
    echo "Run 'cargo fmt' or './scripts/format.sh' to fix formatting."
    exit 1
fi
echo "✅ Formatting OK"
echo ""

# 2. Clippy linting
echo "Running clippy linting..."
cargo clippy --workspace --all-targets -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Linting failed!"
    echo "Fix clippy warnings above or run './scripts/lint.sh' for details."
    exit 1
fi
echo "✅ Linting OK"
echo ""

# 3. Quick tests (skip slow integration tests)
echo "Running quick tests..."
cargo test --workspace --lib
if [ $? -ne 0 ]; then
    echo "❌ Tests failed!"
    echo "Fix failing tests or run './scripts/test.sh' for full output."
    exit 1
fi
echo "✅ Tests OK"
echo ""

echo "✅ All pre-commit checks passed! Proceeding with commit..."
EOF

# Make executable
chmod +x "$PRE_COMMIT_DEST"

echo "✅ Pre-commit hook installed"
echo ""

echo "=== Installation Complete ==="
echo ""
echo "The following hooks are now active:"
echo "  • pre-commit: Validates code format, linting, and tests before commit"
echo "  • pre-push: Validates documentation before pushing"
echo ""
echo "What this means:"
echo "  ✅ Code quality checks run automatically before every commit"
echo "  ✅ Markdown linting runs automatically before every push"
echo "  ✅ Catches issues before CI runs"
echo "  ✅ Can be bypassed with: git commit/push --no-verify"
echo ""
echo "To uninstall:"
echo "  rm .git/hooks/pre-commit"
echo "  rm .git/hooks/pre-push"
echo ""
