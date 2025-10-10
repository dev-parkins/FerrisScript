#!/usr/bin/env bash
# FerrisScript Test Runner
# Convenience script for running test harness examples

set -e

# Color output
GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

function print_success {
    echo -e "${GREEN}✅ $1${NC}"
}

function print_error {
    echo -e "${RED}❌ $1${NC}"
}

function print_info {
    echo -e "${CYAN}ℹ️  $1${NC}"
}

function print_warning {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Parse arguments
SCRIPT=""
ALL_FLAG=""
FAST_FLAG=false
VERBOSE_FLAG=""
FILTER=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --script)
            SCRIPT="--script $2"
            shift 2
            ;;
        --all)
            ALL_FLAG="--all"
            shift
            ;;
        --fast)
            FAST_FLAG=true
            shift
            ;;
        --verbose)
            VERBOSE_FLAG="--verbose"
            shift
            ;;
        --filter)
            FILTER="--filter $2"
            shift 2
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Usage: $0 [--script PATH] [--all] [--fast] [--verbose] [--filter PATTERN]"
            exit 1
            ;;
    esac
done

# Get project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# Build the test harness in release mode if not in fast mode
if [ "$FAST_FLAG" = false ]; then
    print_info "Building test harness in release mode..."
    cargo build --release -p ferrisscript_test_harness
    print_success "Build complete"
    echo ""
fi

# Build the command
TEST_CMD="cargo run --release --bin ferris-test -- $SCRIPT $ALL_FLAG $FILTER $VERBOSE_FLAG"

# Display what we're running
print_info "Running: $TEST_CMD"
echo ""

# Execute the command
if $TEST_CMD; then
    echo ""
    print_success "All tests passed!"
    exit 0
else
    EXIT_CODE=$?
    echo ""
    print_error "Tests failed with exit code $EXIT_CODE"
    exit $EXIT_CODE
fi
