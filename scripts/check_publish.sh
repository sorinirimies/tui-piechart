#!/bin/bash

echo "🔍 Checking tui-slider for publish readiness..."
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

errors=0

# Check formatting
echo -n "Checking code formatting... "
if cargo fmt -- --check > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    errors=$((errors + 1))
fi

# Check clippy
echo -n "Checking clippy... "
if cargo clippy --lib -- -D warnings > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    errors=$((errors + 1))
fi

# Run tests
echo -n "Running tests... "
if cargo test --all-features > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    errors=$((errors + 1))
fi

# Build documentation
echo -n "Building documentation... "
if cargo doc --no-deps > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    errors=$((errors + 1))
fi

# Build all examples
echo -n "Building examples... "
if cargo build --examples > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    errors=$((errors + 1))
fi

# Check required files
echo -n "Checking required files... "
missing=0
for file in README.md LICENSE Cargo.toml CHANGELOG.md; do
    if [ ! -f "$file" ]; then
        echo -e "${RED}Missing: $file${NC}"
        missing=$((missing + 1))
    fi
done
if [ $missing -eq 0 ]; then
    echo -e "${GREEN}✓${NC}"
else
    errors=$((errors + 1))
fi

echo ""
if [ $errors -eq 0 ]; then
    echo -e "${GREEN}✓ All checks passed! Ready to publish.${NC}"
    exit 0
else
    echo -e "${RED}✗ $errors check(s) failed. Please fix before publishing.${NC}"
    exit 1
fi
