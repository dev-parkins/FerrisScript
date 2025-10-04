#!/usr/bin/env bash
# FerrisScript Benchmark Runner
# Runs performance benchmarks for the compiler

set -e  # Exit on error

echo "=========================================="
echo "FerrisScript Benchmark Suite"
echo "=========================================="
echo ""

# Check if criterion is available
if ! cargo bench --help 2>&1 | grep -q "criterion"; then
    echo "⚠️  Note: Benchmarks require 'criterion' in Cargo.toml"
fi

echo "Running compiler benchmarks..."
cargo bench --package ferrisscript_compiler

echo ""
echo "=========================================="
echo "✅ Benchmarks complete!"
echo "Results saved to: target/criterion/"
echo "=========================================="
