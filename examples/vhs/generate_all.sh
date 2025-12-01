#!/usr/bin/env bash
# Generate all VHS demo GIFs for tui-piechart examples
# This script runs all VHS tapes to create demo GIFs for documentation
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

# List of all VHS tapes
TAPES=(
    "piechart"
    "border_styles"
    "legend_positioning"
    "legend_alignment"
    "title_positioning"
    "title_styles"
    "high_resolution"
    "custom_symbols"
    "symbols_circles_squares"
    "symbols_shades_bars"
    "symbols_stars_hearts"
    "symbols_triangles_hexagons"
)

# Generate each tape
TOTAL=${#TAPES[@]}
CURRENT=0

for tape in "${TAPES[@]}"; do
    CURRENT=$((CURRENT + 1))
    TAPE_FILE="examples/vhs/${tape}.tape"

    if [ ! -f "$TAPE_FILE" ]; then
        echo -e "${YELLOW}⚠ Skipping ${tape}.tape (not found)${NC}"
        continue
    fi

    echo -e "${BLUE}[${CURRENT}/${TOTAL}] Generating ${tape}.gif...${NC}"

    if vhs "$TAPE_FILE"; then
        echo -e "${GREEN}✓ Generated ${tape}.gif${NC}"
    else
        echo -e "${YELLOW}⚠ Failed to generate ${tape}.gif${NC}"
    fi
    echo
done

# Summary
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✓ Demo generation complete!${NC}"
echo
echo "Generated GIFs in: examples/vhs/output/"
ls -lh examples/vhs/output/*.gif | awk '{printf "  • %s (%s)\n", $9, $5}'
echo
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
