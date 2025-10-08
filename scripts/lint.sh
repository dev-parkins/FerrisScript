#!/usr/bin/env bash
# FerrisScript Linting (Bash)
# Runs cargo clippy with strict warnings

set -e

echo "=========================================="
echo "FerrisScript Linting (Clippy)"
echo "=========================================="
echo ""

# Run clippy on all workspace crates
echo "Running clippy on workspace..."
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo ""
echo "=========================================="
echo "✅ All linting checks passed!"
echo "=========================================="
