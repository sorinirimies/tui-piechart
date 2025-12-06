#!/usr/bin/env bash
# Generate all VHS demo GIFs for tui-piechart examples
# This script automatically discovers and runs all VHS tapes to create demo GIFs
#
# Prerequisites:
# - VHS installed (https://github.com/charmbracelet/vhs)
# - Examples built with: cargo build --examples --release
#
# Usage: ./examples/vhs/generate_all.sh

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}  TUI PieChart - VHS Demo Generator${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo

# Check if VHS is installed
if ! command -v vhs &> /dev/null; then
    echo -e "${YELLOW}⚠ VHS not found!${NC}"
    echo "Install VHS from: https://github.com/charmbracelet/vhs"
    echo "  macOS: brew install vhs"
    echo "  Linux: see https://github.com/charmbracelet/vhs#installation"
    exit 1
fi

echo -e "${GREEN}✓ VHS found: $(vhs --version)${NC}"
echo

# Change to project root
cd "$PROJECT_ROOT"

# Build examples in release mode for better performance
echo -e "${BLUE}Building examples...${NC}"
cargo build --examples --release
echo -e "${GREEN}✓ Examples built${NC}"
echo

# Create output directory if it doesn't exist
mkdir -p examples/vhs/output

# Dynamically find all .tape files
TAPE_FILES=($(find examples/vhs -maxdepth 1 -name "*.tape" -type f | sort))

if [ ${#TAPE_FILES[@]} -eq 0 ]; then
    echo -e "${YELLOW}⚠ No .tape files found in examples/vhs/${NC}"
    exit 0
fi

echo -e "${BLUE}Found ${#TAPE_FILES[@]} VHS tape(s)${NC}"
echo

# Generate each tape
TOTAL=${#TAPE_FILES[@]}
CURRENT=0
SUCCEEDED=0
FAILED=0

for tape_path in "${TAPE_FILES[@]}"; do
    CURRENT=$((CURRENT + 1))
    tape_name=$(basename "$tape_path" .tape)

    echo -e "${BLUE}[${CURRENT}/${TOTAL}] Generating ${tape_name}.gif...${NC}"

    if vhs "$tape_path"; then
        echo -e "${GREEN}✓ Generated ${tape_name}.gif${NC}"
        SUCCEEDED=$((SUCCEEDED + 1))
    else
        echo -e "${YELLOW}⚠ Failed to generate ${tape_name}.gif${NC}"
        FAILED=$((FAILED + 1))
    fi
    echo
done

# Summary
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✓ Demo generation complete!${NC}"
echo -e "  Succeeded: ${GREEN}${SUCCEEDED}${NC}"
if [ $FAILED -gt 0 ]; then
    echo -e "  Failed: ${YELLOW}${FAILED}${NC}"
fi
echo

if [ $SUCCEEDED -gt 0 ]; then
    echo "Generated GIFs in: examples/vhs/output/"
    ls -lh examples/vhs/output/*.gif 2>/dev/null | awk '{printf "  • %s (%s)\n", $9, $5}' || true
    echo
fi

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
