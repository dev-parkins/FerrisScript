#!/usr/bin/env bash
# Git pre-push hook for FerrisScript
# Runs documentation linting before pushing to catch issues early
#
# Installation (OPTIONAL):
#   1. Copy to .git/hooks/pre-push: cp scripts/pre-push.sh .git/hooks/pre-push
#   2. Make executable: chmod +x .git/hooks/pre-push
#   OR use the install script: ./scripts/install-git-hooks.sh
#
# To bypass this hook (not recommended):
#   git push --no-verify

set -e

echo ""
echo "🔍 Running pre-push documentation checks..."
echo ""

# Check if any .md files are being pushed
MD_FILES_CHANGED=$(git diff --name-only @{u}.. 2>/dev/null | grep '\.md$' || true)

if [ -z "$MD_FILES_CHANGED" ]; then
    echo "✅ No markdown files changed, skipping documentation checks"
    exit 0
fi

echo "📄 Markdown files changed:"
echo "$MD_FILES_CHANGED" | while read -r file; do
    echo "   - $file"
done
echo ""

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "⚠️  Node.js not installed - skipping documentation checks"
    echo "   Install Node.js to enable pre-push documentation validation"
    echo "   Download: https://nodejs.org/"
    echo ""
    echo "🚀 Pushing anyway (checks will run in CI)..."
    exit 0
fi

# Check if npm packages are installed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing npm dependencies..."
    npm install --silent
    if [ $? -ne 0 ]; then
        echo ""
        echo "❌ Failed to install npm dependencies"
        echo "   Run manually: npm install"
        echo ""
        echo "🚀 Pushing anyway (checks will run in CI)..."
        exit 0
    fi
fi

# Run documentation linting
echo "🔧 Running markdownlint..."
npx markdownlint '**/*.md' --ignore node_modules --ignore target --dot
LINT_EXIT=$?

if [ $LINT_EXIT -ne 0 ]; then
    echo ""
    echo "❌ Documentation linting failed!"
    echo ""
    echo "To fix automatically, run:"
    echo "   npm run docs:fix"
    echo ""
    echo "To bypass this check (not recommended):"
    echo "   git push --no-verify"
    echo ""
    exit 1
fi

echo "✅ Documentation checks passed!"
echo ""
exit 0
