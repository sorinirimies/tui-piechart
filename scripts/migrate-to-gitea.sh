#!/usr/bin/env bash
# Universal Gitea Migration Script
# Migrates any Git project to support dual hosting with GitHub and Gitea
# Usage: ./migrate-to-gitea.sh [project-dir] [gitea-url]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
error() {
    echo -e "${RED}❌ Error: $1${NC}" >&2
    exit 1
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

heading() {
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Show usage
show_usage() {
    cat << EOF
Usage: $0 [project-dir] [gitea-url]

Examples:
  $0 /home/user/Projects/my-project git@gitea.example.com:user/my-project.git
  $0 . git@gitea.example.com:user/current-project.git
  $0  (interactive mode - will prompt for inputs)

This script will:
  1. Add Gitea as a remote
  2. Copy Gitea setup files from tui-slider template
  3. Update justfile with Gitea commands
  4. Configure SSH and test connection
  5. Optionally push all code to Gitea

Requirements:
  - Git installed
  - SSH keys configured
  - tui-slider as template (in parent directory)
EOF
}

# Parse arguments
if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    show_usage
    exit 0
fi

# Get project directory
if [ -n "$1" ]; then
    PROJECT_DIR="$1"
else
    echo "Enter project directory (or '.' for current):"
    read -r PROJECT_DIR
fi

# Resolve to absolute path
PROJECT_DIR=$(cd "$PROJECT_DIR" && pwd)

# Get project name from directory
PROJECT_NAME=$(basename "$PROJECT_DIR")

# Get Gitea URL
if [ -n "$2" ]; then
    GITEA_URL="$2"
else
    echo "Enter Gitea repository URL (SSH format):"
    echo "Example: git@gitea.example.com:username/${PROJECT_NAME}.git"
    read -r GITEA_URL
fi

# Validate inputs
if [ ! -d "$PROJECT_DIR" ]; then
    error "Project directory does not exist: $PROJECT_DIR"
fi

if [ -z "$GITEA_URL" ]; then
    error "Gitea URL is required"
fi

# Check if it's a git repository
if [ ! -d "$PROJECT_DIR/.git" ]; then
    error "Not a git repository: $PROJECT_DIR"
fi

# Extract Gitea hostname from URL
if [[ "$GITEA_URL" == git@* ]]; then
    USE_SSH=true
    GITEA_HOST=$(echo "$GITEA_URL" | sed 's/git@\([^:]*\):.*/\1/')
else
    USE_SSH=false
    warning "HTTPS URL detected. SSH is strongly recommended!"
fi

# Find tui-slider template directory
TEMPLATE_DIR=""
if [ -d "$(dirname "$PROJECT_DIR")/tui-slider" ]; then
    TEMPLATE_DIR="$(dirname "$PROJECT_DIR")/tui-slider"
elif [ -d "$HOME/Projects/tui-slider" ]; then
    TEMPLATE_DIR="$HOME/Projects/tui-slider"
fi

if [ -z "$TEMPLATE_DIR" ] || [ ! -d "$TEMPLATE_DIR" ]; then
    warning "tui-slider template not found. Will create minimal setup."
    USE_TEMPLATE=false
else
    info "Using template from: $TEMPLATE_DIR"
    USE_TEMPLATE=true
fi

# Start migration
heading "Migrating $PROJECT_NAME to Gitea"

echo ""
info "Project: $PROJECT_NAME"
info "Directory: $PROJECT_DIR"
info "Gitea URL: $GITEA_URL"
info "Gitea Host: $GITEA_HOST"
echo ""

# Change to project directory
cd "$PROJECT_DIR"

# Check SSH if using SSH URL
if [ "$USE_SSH" = true ]; then
    heading "Checking SSH Configuration"

    # Check for SSH keys
    if [ ! -f ~/.ssh/id_rsa ] && [ ! -f ~/.ssh/id_ed25519 ] && [ ! -f ~/.ssh/id_ecdsa ]; then
        warning "No SSH keys found!"
        echo ""
        info "Generate SSH key with:"
        echo "  ssh-keygen -t ed25519 -C \"your_email@example.com\""
        echo ""
        read -p "Continue anyway? (y/N) " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            error "SSH keys required. Please set up SSH first."
        fi
    else
        success "SSH keys found"

        # Test SSH connection
        info "Testing SSH connection to $GITEA_HOST..."
        if ssh -o ConnectTimeout=5 -T git@"$GITEA_HOST" 2>&1 | grep -q "successfully authenticated\|Hi there"; then
            success "SSH connection successful!"
        else
            warning "Could not verify SSH connection"
            info "Make sure your SSH key is added to Gitea:"
            echo "  1. Copy: cat ~/.ssh/id_ed25519.pub"
            echo "  2. Add in Gitea: Settings → SSH/GPG Keys"
            echo ""
        fi
    fi
fi

# Add Gitea remote
heading "Adding Gitea Remote"

if git remote | grep -q "^gitea$"; then
    warning "Gitea remote already exists"
    info "Updating URL..."
    git remote set-url gitea "$GITEA_URL"
    success "Gitea remote URL updated"
else
    git remote add gitea "$GITEA_URL"
    success "Gitea remote added"
fi

# Show remotes
info "Configured remotes:"
git remote -v | grep -E "^(origin|gitea)" | sed 's/^/  /'
echo ""

# Copy template files if available
if [ "$USE_TEMPLATE" = true ]; then
    heading "Copying Template Files"

    FILES_TO_COPY=(
        "SSH_SETUP.md"
        "GITEA_SETUP.md"
        "DUAL_HOSTING.md"
    )

    for file in "${FILES_TO_COPY[@]}"; do
        if [ -f "$TEMPLATE_DIR/$file" ]; then
            cp "$TEMPLATE_DIR/$file" "$PROJECT_DIR/"
            success "Copied $file"
        fi
    done

    # Copy scripts
    if [ ! -d "$PROJECT_DIR/scripts" ]; then
        mkdir -p "$PROJECT_DIR/scripts"
    fi

    if [ -f "$TEMPLATE_DIR/scripts/setup-gitea.sh" ]; then
        cp "$TEMPLATE_DIR/scripts/setup-gitea.sh" "$PROJECT_DIR/scripts/"
        chmod +x "$PROJECT_DIR/scripts/setup-gitea.sh"
        success "Copied setup-gitea.sh"
    fi
fi

# Update or create justfile
heading "Updating Justfile"

if [ -f "$PROJECT_DIR/justfile" ]; then
    info "Justfile exists, checking for Gitea commands..."

    if grep -q "push-gitea" "$PROJECT_DIR/justfile"; then
        success "Gitea commands already present in justfile"
    else
        info "Adding Gitea commands to justfile..."

        # Backup original
        cp "$PROJECT_DIR/justfile" "$PROJECT_DIR/justfile.backup"
        success "Backed up justfile to justfile.backup"

        # Add Gitea commands
        cat >> "$PROJECT_DIR/justfile" << 'JUSTFILE_APPEND'

# ============================================================================
# Gitea Dual-Hosting Commands
# ============================================================================

# Git: push to GitHub (origin)
push:
    git push origin main

# Git: pull from GitHub (origin)
pull:
    git pull origin main

# Git: push to Gitea
push-gitea:
    git push gitea main

# Git: pull from Gitea
pull-gitea:
    git pull gitea main

# Git: push to both GitHub and Gitea
push-all:
    git push origin main
    git push gitea main
    @echo "✅ Pushed to both GitHub and Gitea!"

# Git: pull from Gitea (fetch and merge)
pull-from-gitea:
    @echo "Pulling from Gitea..."
    git pull gitea main
    @echo "✅ Pulled from Gitea!"

# Git: push tags to GitHub
push-tags:
    git push origin --tags

# Git: push tags to both remotes
push-tags-all:
    git push origin --tags
    git push gitea --tags
    @echo "✅ Tags pushed to both GitHub and Gitea!"

# Full release workflow: bump version and push to GitHub
release version: (bump version)
    @echo "Pushing to GitHub..."
    git push origin main
    git push origin v{{version}}
    @echo "✅ Release v{{version}} complete on GitHub!"

# Full release workflow: bump version and push to Gitea
release-gitea version: (bump version)
    @echo "Pushing to Gitea..."
    git push gitea main
    git push gitea v{{version}}
    @echo "✅ Release v{{version}} complete on Gitea!"

# Full release workflow: bump version and push to both GitHub and Gitea
release-all version: (bump version)
    @echo "Pushing to both GitHub and Gitea..."
    git push origin main
    git push gitea main
    git push origin v{{version}}
    git push gitea v{{version}}
    @echo "✅ Release v{{version}} complete on both remotes!"

# Push release to both GitHub and Gitea (without bumping)
push-release-all:
    @echo "Pushing release to both GitHub and Gitea..."
    git push origin main
    git push gitea main
    git push origin --tags
    git push gitea --tags
    @echo "✅ Release pushed to both remotes!"

# Sync Gitea with GitHub (force)
sync-gitea:
    @echo "Syncing Gitea with GitHub..."
    git push gitea main --force
    git push gitea --tags --force
    @echo "✅ Gitea synced!"

# Show configured remotes
remotes:
    @echo "Configured git remotes:"
    @git remote -v

# Setup Gitea remote (provide your Gitea URL)
setup-gitea url:
    @echo "Adding Gitea remote..."
    git remote add gitea {{url}}
    @echo "✅ Gitea remote added!"
    @echo "Test with: git push gitea main"
JUSTFILE_APPEND

        success "Added Gitea commands to justfile"
        info "Run 'just --list' to see new commands"
    fi
else
    warning "No justfile found, creating comprehensive one..."

    cat > "$PROJECT_DIR/justfile" << 'JUSTFILE_CREATE'
# $PROJECT_NAME - Project automation with dual GitHub/Gitea hosting
# Install just: cargo install just
# Install git-cliff: cargo install git-cliff
# Usage: just <task>

# Default task - show available commands
default:
    @just --list

# Install required tools (just, git-cliff)
install-tools:
    @echo "Installing required tools..."
    @command -v just >/dev/null 2>&1 || cargo install just
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff
    @echo "✅ All tools installed!"

# Build the project
build:
    cargo build

# Build release version
build-release:
    cargo build --release

# Run the example (customize as needed)
run:
    cargo run --example main

# Run tests
test:
    cargo test

# Run tests with coverage
test-coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Check if code is formatted
fmt-check:
    cargo fmt --check

# Run clippy linter
clippy:
    cargo clippy -- -D warnings

# Run all checks (fmt, clippy, test)
check-all: fmt-check clippy test
    @echo "✅ All checks passed!"

# Clean build artifacts
clean:
    cargo clean

# Check if git-cliff is installed
check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { echo "❌ git-cliff not found. Install with: cargo install git-cliff"; exit 1; }

# Generate full changelog from all tags
changelog: check-git-cliff
    @echo "Generating full changelog..."
    git-cliff -o CHANGELOG.md
    @echo "✅ Changelog generated!"

# Generate changelog for unreleased commits only
changelog-unreleased: check-git-cliff
    @echo "Generating unreleased changelog..."
    git-cliff --unreleased --prepend CHANGELOG.md
    @echo "✅ Unreleased changelog generated!"

# Generate changelog for specific version tag
changelog-version version: check-git-cliff
    @echo "Generating changelog for version {{version}}..."
    git-cliff --tag v{{version}} -o CHANGELOG.md
    @echo "✅ Changelog generated for version {{version}}!"

# Preview changelog without writing to file
changelog-preview: check-git-cliff
    @git-cliff

# Preview unreleased changes
changelog-preview-unreleased: check-git-cliff
    @git-cliff --unreleased

# Generate changelog for latest tag only
changelog-latest: check-git-cliff
    @echo "Generating changelog for latest tag..."
    git-cliff --latest -o CHANGELOG.md
    @echo "✅ Latest changelog generated!"

# Update changelog with all commits (force regenerate)
changelog-update: check-git-cliff
    @echo "Regenerating complete changelog from all tags..."
    git-cliff --output CHANGELOG.md
    @echo "✅ Changelog updated from all git history!"

# Bump version (usage: just bump 0.2.0)
bump version: check-git-cliff
    @echo "Bumping version to {{version}}..."
    @./scripts/bump_version.sh {{version}}

# Quick release: format, check, test, and build
release-check: fmt clippy test build-release
    @echo "✅ Ready for release!"

# Publish to crates.io (dry run)
publish-dry:
    cargo publish --dry-run

# Publish to crates.io
publish:
    cargo publish

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated

# Generate documentation
doc:
    cargo doc --no-deps --open

# Watch and auto-run on file changes (requires cargo-watch)
watch:
    cargo watch -x run

# Git: commit current changes
commit message:
    git add .
    git commit -m "{{message}}"

# Git: push to GitHub (origin)
push:
    git push origin main

# Git: pull from GitHub (origin)
pull:
    git pull origin main

# Git: push to Gitea
push-gitea:
    git push gitea main

# Git: pull from Gitea
pull-gitea:
    git pull gitea main

# Git: push to both GitHub and Gitea
push-all:
    git push origin main
    git push gitea main
    @echo "✅ Pushed to both GitHub and Gitea!"

# Git: pull from Gitea (fetch and merge)
pull-from-gitea:
    @echo "Pulling from Gitea..."
    git pull gitea main
    @echo "✅ Pulled from Gitea!"

# Git: push tags to GitHub
push-tags:
    git push origin --tags

# Git: push tags to both remotes
push-tags-all:
    git push origin --tags
    git push gitea --tags
    @echo "✅ Tags pushed to both GitHub and Gitea!"

# Full release workflow: bump version and push to GitHub
release version: (bump version)
    @echo "Pushing to GitHub..."
    git push origin main
    git push origin v{{version}}
    @echo "✅ Release v{{version}} complete on GitHub!"

# Full release workflow: bump version and push to Gitea
release-gitea version: (bump version)
    @echo "Pushing to Gitea..."
    git push gitea main
    git push gitea v{{version}}
    @echo "✅ Release v{{version}} complete on Gitea!"

# Full release workflow: bump version and push to both GitHub and Gitea
release-all version: (bump version)
    @echo "Pushing to both GitHub and Gitea..."
    git push origin main
    git push gitea main
    git push origin v{{version}}
    git push gitea v{{version}}
    @echo "✅ Release v{{version}} complete on both remotes!"

# Push release to both GitHub and Gitea (without bumping)
push-release-all:
    @echo "Pushing release to both GitHub and Gitea..."
    git push origin main
    git push gitea main
    git push origin --tags
    git push gitea --tags
    @echo "✅ Release pushed to both remotes!"

# Sync Gitea with GitHub (force)
sync-gitea:
    @echo "Syncing Gitea with GitHub..."
    git push gitea main --force
    git push gitea --tags --force
    @echo "✅ Gitea synced!"

# Show configured remotes
remotes:
    @echo "Configured git remotes:"
    @git remote -v

# Setup Gitea remote (provide your Gitea URL)
setup-gitea url:
    @echo "Adding Gitea remote..."
    git remote add gitea {{url}}
    @echo "✅ Gitea remote added!"
    @echo "Test with: git push gitea main"

# Show current version
version:
    @grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'

# Show git-cliff info
cliff-info:
    @echo "Git-cliff configuration:"
    @echo "  Config file: cliff.toml"
    @echo "  Installed: $(command -v git-cliff >/dev/null 2>&1 && echo '✅ Yes' || echo '❌ No (run: just install-tools)')"
    @command -v git-cliff >/dev/null 2>&1 && git-cliff --version || true

# Show project info
info:
    @echo "Project: $PROJECT_NAME"
    @echo "Version: $(just version)"
    @echo "License: MIT"

# View changelog
view-changelog:
    @cat CHANGELOG.md
JUSTFILE_CREATE

    # Replace $PROJECT_NAME with actual project name
    sed -i "s/\$PROJECT_NAME/$PROJECT_NAME/g" "$PROJECT_DIR/justfile"

    success "Created comprehensive justfile with all commands"
    info "Customize the 'run' command and other project-specific settings"
fi

# Create bump_version.sh script if it doesn't exist
heading "Checking Version Bump Script"

if [ ! -d "$PROJECT_DIR/scripts" ]; then
    mkdir -p "$PROJECT_DIR/scripts"
    success "Created scripts directory"
fi

if [ -f "$PROJECT_DIR/scripts/bump_version.sh" ]; then
    success "bump_version.sh already exists"
else
    info "Creating bump_version.sh script..."

    cat > "$PROJECT_DIR/scripts/bump_version.sh" << 'BUMP_SCRIPT'
#!/bin/bash
# Automated version bump script
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
sed -i "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" Cargo.toml

# Update README.md version badge if it exists
if [ -f README.md ]; then
    echo -e "${GREEN}Updating README.md...${NC}"
    if grep -q "version-[0-9]*\.[0-9]*\.[0-9]*-blue" README.md 2>/dev/null; then
        sed -i "s/version-[0-9]*\.[0-9]*\.[0-9]*-blue/version-${NEW_VERSION}-blue/" README.md
    fi
fi

# Update Cargo.lock
echo -e "${GREEN}Updating Cargo.lock...${NC}"
PROJECT_NAME=$(grep '^name = ' Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/')
cargo update -p "$PROJECT_NAME" || cargo build

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
git add Cargo.toml Cargo.lock CHANGELOG.md
if [ -f README.md ]; then
    git add README.md
fi

echo -e "${GREEN}Creating commit...${NC}"
git commit -m "chore: bump version to ${NEW_VERSION}

- Update version in Cargo.toml and README.md
- Update Cargo.lock
- Generate updated CHANGELOG.md"

echo -e "${GREEN}Creating tag...${NC}"
git tag -a "v${NEW_VERSION}" -m "Release v${NEW_VERSION}"

echo -e "${YELLOW}Changes committed and tagged locally.${NC}"
echo -e "${YELLOW}To push to remote, use justfile commands:${NC}"
echo -e "  just release ${NEW_VERSION}         # Push to GitHub"
echo -e "  just release-gitea ${NEW_VERSION}   # Push to Gitea"
echo -e "  just release-all ${NEW_VERSION}     # Push to both"
echo ""
echo -e "${GREEN}Version bump complete! 🚀${NC}"
BUMP_SCRIPT

    chmod +x "$PROJECT_DIR/scripts/bump_version.sh"
    success "Created bump_version.sh script"
    info "Script location: scripts/bump_version.sh"
fi

# Test Gitea connection
heading "Testing Gitea Connection"

info "Testing connection to Gitea repository..."
if git ls-remote gitea > /dev/null 2>&1; then
    success "Successfully connected to Gitea repository!"
else
    warning "Could not connect to Gitea repository"
    info "This is normal if the repository doesn't exist yet"
    echo ""
    info "To create the repository on Gitea:"
    echo "  1. Log in to your Gitea instance"
    echo "  2. Create repository: $PROJECT_NAME"
    echo "  3. Do NOT initialize with README"
    echo "  4. Run: just push-gitea"
    echo ""
fi

# Offer to push
heading "Push to Gitea"

read -p "Do you want to push all branches and tags to Gitea now? (y/N) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    info "Pushing to Gitea..."

    if git push gitea --all 2>&1; then
        success "All branches pushed to Gitea"
    else
        warning "Failed to push branches (repository might not exist yet)"
    fi

    if git push gitea --tags 2>&1; then
        success "All tags pushed to Gitea"
    else
        warning "Failed to push tags"
    fi
fi

# Create .gitea directory for Gitea Actions
echo ""
read -p "Set up Gitea Actions (CI/CD)? (y/N) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ ! -d ".gitea" ]; then
        mkdir -p .gitea/workflows

        if [ -d ".github/workflows" ]; then
            info "Copying workflows from .github to .gitea..."
            cp -r .github/workflows/* .gitea/workflows/ 2>/dev/null || true
            success "Workflows copied to .gitea/workflows/"
        else
            success ".gitea/workflows directory created"
            info "Add your workflow files to .gitea/workflows/"
        fi
    else
        success ".gitea directory already exists"
    fi
fi

# Summary
echo ""
heading "Migration Complete! 🎉"

echo ""
success "Project $PROJECT_NAME migrated to Gitea!"
echo ""

info "What was done:"
echo "  ✓ Added Gitea remote: $GITEA_URL"
echo "  ✓ Copied documentation files"
echo "  ✓ Updated/created justfile with Gitea commands"
if [ -d ".gitea" ]; then
    echo "  ✓ Set up .gitea directory for CI/CD"
fi
echo ""

info "Quick commands:"
echo "  just push             # Push to GitHub only"
echo "  just push-gitea       # Push to Gitea only"
echo "  just push-all         # Push to both GitHub and Gitea"
echo "  just push-tags-all    # Push tags to both remotes"
echo "  just sync-gitea       # Sync Gitea with GitHub"
echo "  just remotes          # Show all remotes"
echo ""
info "Release commands:"
echo "  just release 0.2.5         # Release to GitHub only"
echo "  just release-gitea 0.2.5   # Release to Gitea only"
echo "  just release-all 0.2.5     # Release to both remotes"
echo ""

info "Documentation:"
if [ -f "GITEA_SETUP.md" ]; then
    echo "  📖 GITEA_SETUP.md     # Quick setup guide"
fi
if [ -f "SSH_SETUP.md" ]; then
    echo "  🔑 SSH_SETUP.md       # SSH configuration"
fi
if [ -f "DUAL_HOSTING.md" ]; then
    echo "  📚 DUAL_HOSTING.md    # Complete dual-hosting guide"
fi
echo ""

info "Next steps:"
echo "  1. Review the documentation files"
echo "  2. Test: just push-gitea"
echo "  3. View all commands: just --list"
echo ""

if [ "$USE_SSH" = true ]; then
    success "SSH configured - no passwords needed! 🔑"
else
    warning "Using HTTPS - you'll need to enter credentials"
    info "Switch to SSH: git remote set-url gitea git@$GITEA_HOST:username/$PROJECT_NAME.git"
fi

echo ""
success "Happy dual-hosting! 🚀"
