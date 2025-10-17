#!/bin/bash

# Test script for Amari MCP Server
# Runs comprehensive test suite with different configurations

set -e

echo "🧪 Amari MCP Test Suite"
echo "======================="

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

run_test() {
    local test_name="$1"
    local test_command="$2"

    echo -e "\n${YELLOW}Running: $test_name${NC}"

    if eval "$test_command"; then
        echo -e "${GREEN}✅ PASSED: $test_name${NC}"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}❌ FAILED: $test_name${NC}"
        ((TESTS_FAILED++))
    fi
}

# Check if PostgreSQL is running (for database tests)
check_postgres() {
    if command -v pg_isready >/dev/null 2>&1; then
        if pg_isready -h localhost -p 5432 >/dev/null 2>&1; then
            echo "✅ PostgreSQL is running"
            return 0
        fi
    fi
    echo "⚠️  PostgreSQL not running - database tests will be skipped"
    return 1
}

# Setup test database if PostgreSQL is available
setup_test_db() {
    if check_postgres; then
        echo "🗄️  Setting up test database..."
        export TEST_DATABASE_URL="postgres://postgres:postgres@localhost:5432/amari_mcp_test"

        # Create test database if it doesn't exist
        createdb amari_mcp_test 2>/dev/null || true

        echo "✅ Test database ready"
        return 0
    fi
    return 1
}

echo "🔍 Checking prerequisites..."

# Check Rust installation
if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ Cargo not found. Please install Rust."
    exit 1
fi

# Setup database for tests
DATABASE_AVAILABLE=$(setup_test_db && echo "true" || echo "false")

echo -e "\n🧹 Cleaning previous builds..."
cargo clean

echo -e "\n📝 Code formatting check..."
run_test "Format check" "cargo fmt --all -- --check"

echo -e "\n🔍 Clippy lints..."
run_test "Clippy (no features)" "cargo clippy --all-targets -- -D warnings"
run_test "Clippy (database)" "cargo clippy --all-targets --features database -- -D warnings"

echo -e "\n🔨 Build tests..."
run_test "Build (no features)" "cargo build"
run_test "Build (database)" "cargo build --features database"
run_test "Build (gpu)" "cargo build --features gpu"
run_test "Build (all features)" "cargo build --features database,gpu"

echo -e "\n🧪 Unit tests..."
run_test "Unit tests (no features)" "cargo test --lib"

if [ "$DATABASE_AVAILABLE" = "true" ]; then
    run_test "Unit tests (database)" "cargo test --lib --features database"
    run_test "Database migration tests" "cargo test database_migration_test --features database"
    run_test "Cayley precompute tests" "cargo test cayley_precompute_test --features database"
else
    echo "⚠️  Skipping database tests - PostgreSQL not available"
fi

echo -e "\n🔧 Integration tests..."
run_test "MCP integration tests" "cargo test mcp_integration_test"
run_test "Cayley table tests" "cargo test cayley_tables_test"

echo -e "\n🏗️  Release build test..."
run_test "Release build" "cargo build --release --features database,gpu"

echo -e "\n📚 Documentation build..."
run_test "Documentation" "cargo doc --all-features --no-deps"

echo -e "\n🔐 Security audit..."
if command -v cargo-audit >/dev/null 2>&1; then
    run_test "Security audit" "cargo audit"
else
    echo "⚠️  cargo-audit not installed - skipping security audit"
    echo "   Install with: cargo install cargo-audit"
fi

# Summary
echo -e "\n📊 Test Summary"
echo "==============="
echo -e "✅ Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "❌ Failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n🎉 ${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "\n💥 ${RED}Some tests failed.${NC}"
    exit 1
fi