#!/usr/bin/env bash
# FerrisScript Test Runner
# Runs all tests in the workspace

set -e  # Exit on error

echo "=========================================="
echo "FerrisScript Test Suite"
echo "=========================================="
echo ""

# Run all workspace tests
echo "Running all workspace tests..."
cargo test --workspace

echo ""
echo "=========================================="
echo "✅ All tests passed!"
echo "=========================================="
