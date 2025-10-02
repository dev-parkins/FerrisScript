#!/usr/bin/env bash
# Documentation Linting Script for FerrisScript
# Usage: ./scripts/lint-docs.sh [--fix]

set -e

FIX_MODE=false

# Parse arguments
if [ "$1" == "--fix" ]; then
    FIX_MODE=true
fi

echo ""
echo "=== FerrisScript Documentation Linting ==="
echo ""

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to run documentation linting."
    echo "   Download from: https://nodejs.org/"
    exit 1
fi

# Check if npm packages are installed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing npm dependencies..."
    npm install
    echo ""
fi

# Run markdownlint
echo "🔍 Step 1/2: Running markdownlint..."
if [ "$FIX_MODE" == true ]; then
    echo "   Mode: Fix (will auto-fix issues)"
    npm run docs:fix
else
    echo "   Mode: Check only"
    npm run docs:lint
fi

MARKDOWNLINT_EXIT=$?
echo ""

# Run markdown-link-check
echo "🔗 Step 2/2: Running markdown-link-check..."
LINK_CHECK_FAILED=false
CHECKED_FILES=0

# Find all .md files, excluding node_modules and target
find . -name "*.md" -not -path "./node_modules/*" -not -path "./target/*" | while read -r file; do
    ((CHECKED_FILES++))
    filename=$(basename "$file")
    echo "   Checking: $filename"
    
    # Run link check and capture output
    output=$(npx markdown-link-check "$file" --config .markdown-link-check.json 2>&1 || true)
    
    # Check for dead links in output
    if echo "$output" | grep -q '\[✖\]'; then
        LINK_CHECK_FAILED=true
        echo "   ❌ Dead links in: $filename"
        # Extract and show dead links
        echo "$output" | grep '\[✖\]' | while read -r line; do
            echo "      $line"
        done
    fi
done

echo ""
echo "   Files checked: $CHECKED_FILES"
if [ "$LINK_CHECK_FAILED" = true ]; then
    echo "   Dead links found!"
    LINK_CHECK_EXIT=1
else
    LINK_CHECK_EXIT=0
fi

echo ""

# Summary
echo "=== Summary ==="
if [ $MARKDOWNLINT_EXIT -eq 0 ] && [ $LINK_CHECK_EXIT -eq 0 ]; then
    echo "✅ All documentation checks passed!"
    exit 0
else
    if [ $MARKDOWNLINT_EXIT -ne 0 ]; then
        echo "❌ Markdownlint found issues"
        if [ "$FIX_MODE" != true ]; then
            echo "   Run with --fix to auto-fix: ./scripts/lint-docs.sh --fix"
        fi
    fi
    if [ $LINK_CHECK_EXIT -ne 0 ]; then
        echo "❌ Broken links found"
    fi
    exit 1
fi
