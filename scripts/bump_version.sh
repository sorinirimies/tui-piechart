#!/bin/bash
# Automated version bump script for Tui Piechart
# Usage: ./scripts/bump_version.sh <new_version>
# Example: ./scripts/bump_version.sh 0.2.5

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version argument is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo "Usage: $0 <version>"
    echo "Example: $0 0.2.5"
    exit 1
fi

NEW_VERSION=$1

# Validate version format (semantic versioning)
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Version must be in format: X.Y.Z (e.g., 0.2.5)"
    exit 1
fi

echo -e "${YELLOW}Bumping version to ${NEW_VERSION}...${NC}"

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -e "Current version: ${CURRENT_VERSION}"
echo -e "New version: ${NEW_VERSION}"

# Ask for confirmation
read -p "Continue? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Aborted${NC}"
    exit 0
fi

# Update Cargo.toml
echo -e "${GREEN}Updating Cargo.toml...${NC}"
sed -i '' "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" Cargo.toml

# Update README.md version badge if it exists
echo -e "${GREEN}Updating README.md...${NC}"
if grep -q "version-[0-9]*\.[0-9]*\.[0-9]*-blue" README.md 2>/dev/null; then
    sed -i '' "s/version-[0-9]*\.[0-9]*\.[0-9]*-blue/version-${NEW_VERSION}-blue/" README.md
fi

# Update Cargo.lock
echo -e "${GREEN}Updating Cargo.lock...${NC}"
cargo update -p tui-piechart

# Check formatting
echo -e "${GREEN}Running cargo fmt...${NC}"
cargo fmt

# Check for issues
echo -e "${GREEN}Running cargo clippy...${NC}"
if ! cargo clippy -- -D warnings; then
    echo -e "${RED}Clippy found issues. Please fix them before continuing.${NC}"
    exit 1
fi

# Run tests
echo -e "${GREEN}Running tests...${NC}"
if ! cargo test --locked --all-features --all-targets; then
    echo -e "${RED}Tests failed. Please fix them before continuing.${NC}"
    exit 1
fi

# Generate changelog
echo -e "${GREEN}Generating CHANGELOG.md...${NC}"
if command -v git-cliff &> /dev/null; then
    git-cliff --tag "v${NEW_VERSION}" -o CHANGELOG.md
    echo -e "${GREEN}Changelog updated${NC}"
else
    echo -e "${YELLOW}Warning: git-cliff not found. Skipping changelog generation.${NC}"
    echo -e "${YELLOW}Install it with: cargo install git-cliff${NC}"
fi

# Git operations
echo -e "${GREEN}Staging changes...${NC}"
git add Cargo.toml Cargo.lock README.md CHANGELOG.md

echo -e "${GREEN}Creating commit...${NC}"
git commit -m "chore: bump version to ${NEW_VERSION}

- Update version in Cargo.toml and README.md
- Update Cargo.lock
- Generate updated CHANGELOG.md"

echo -e "${GREEN}Creating tag...${NC}"
git tag -a "v${NEW_VERSION}" -m "Release v${NEW_VERSION}"

echo -e "${YELLOW}Changes committed and tagged locally.${NC}"
echo -e "${YELLOW}To push to remote, run:${NC}"
echo -e "  git push origin main"
echo -e "  git push origin v${NEW_VERSION}"
echo ""
echo -e "${GREEN}Version bump complete! 🚀${NC}"
