#!/usr/bin/env bash
# Run test coverage analysis with cargo-tarpaulin
# Usage: ./scripts/coverage.sh

set -e

echo "🔍 Running test coverage analysis..."

# Check if tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "❌ cargo-tarpaulin not found. Installing..."
    cargo install cargo-tarpaulin
fi

# Create coverage output directory
mkdir -p target/coverage

# Run coverage
echo "📊 Analyzing coverage across workspace..."
cargo tarpaulin --workspace --out Html --out Lcov --out Stdout --output-dir target/coverage

echo ""
echo "✅ Coverage analysis complete!"
echo ""
echo "📄 Reports generated:"
echo "  - HTML: target/coverage/tarpaulin-report.html"
echo "  - LCOV: target/coverage/lcov.info"
echo ""
echo "🌐 Open HTML report:"
echo "  xdg-open target/coverage/tarpaulin-report.html  # Linux"
echo "  open target/coverage/tarpaulin-report.html      # macOS"
