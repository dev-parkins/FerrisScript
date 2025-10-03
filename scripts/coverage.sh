#!/usr/bin/env bash
# Run test coverage analysis with cargo-llvm-cov
# Usage: ./scripts/coverage.sh
#
# Note: Switched from tarpaulin to llvm-cov for better cross-platform compatibility
# See docs/COVERAGE_SETUP_NOTES.md for details

set -e

echo "🔍 Running test coverage analysis..."

# Check if llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "❌ cargo-llvm-cov not found. Installing..."
    
    # Install llvm-tools-preview
    echo "📦 Installing llvm-tools-preview component..."
    rustup component add llvm-tools-preview
    
    # Install cargo-llvm-cov
    echo "📦 Installing cargo-llvm-cov..."
    cargo install cargo-llvm-cov
fi

# Create coverage output directory
mkdir -p target/coverage

# Run coverage
echo "📊 Analyzing coverage across workspace..."
cargo llvm-cov --workspace --html --output-dir target/coverage
cargo llvm-cov --workspace --lcov --output-path target/coverage/lcov.info

echo ""
echo "✅ Coverage analysis complete!"
echo ""
echo "📄 Reports generated:"
echo "  - HTML: target/coverage/html/index.html"
echo "  - LCOV: target/coverage/lcov.info"
echo ""
echo "🌐 Open HTML report:"
echo "  xdg-open target/coverage/html/index.html  # Linux"
echo "  open target/coverage/html/index.html      # macOS"
