#!/usr/bin/env bash
# FerrisScript Code Formatter
# Formats all Rust code in the workspace

set -e  # Exit on error

echo "=========================================="
echo "FerrisScript Code Formatter"
echo "=========================================="
echo ""

echo "Formatting all Rust code..."
cargo fmt --all

echo ""
echo "=========================================="
echo "✅ Code formatted successfully!"
echo "Tip: Run 'cargo fmt -- --check' to verify without modifying files"
echo "=========================================="
