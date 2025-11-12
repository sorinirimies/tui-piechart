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

# Git: push to Gitea
push-gitea:
    git push gitea main

# Git: push to both GitHub and Gitea
push-all:
    git push origin main
    git push gitea main
    @echo "✅ Pushed to both GitHub and Gitea!"

# Git: push tags to both remotes
push-tags-all:
    git push origin --tags
    git push gitea --tags
    @echo "✅ Tags pushed to both GitHub and Gitea!"

# Push release to both GitHub and Gitea
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
    warning "No justfile found, creating basic one..."

    cat > "$PROJECT_DIR/justfile" << 'JUSTFILE_CREATE'
# Justfile for project automation
# Usage: just <task>

# Default task - show available commands
default:
    @just --list

# Push to GitHub only
push:
    git push origin main

# Push to Gitea only
push-gitea:
    git push gitea main

# Push to both GitHub and Gitea
push-all:
    git push origin main
    git push gitea main
    @echo "✅ Pushed to both GitHub and Gitea!"

# Push tags to both remotes
push-tags-all:
    git push origin --tags
    git push gitea --tags
    @echo "✅ Tags pushed to both GitHub and Gitea!"

# Show configured remotes
remotes:
    @echo "Configured git remotes:"
    @git remote -v

# Sync Gitea with GitHub (force)
sync-gitea:
    @echo "Syncing Gitea with GitHub..."
    git push gitea main --force
    git push gitea --tags --force
    @echo "✅ Gitea synced!"
JUSTFILE_CREATE

    success "Created justfile with Gitea commands"
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
echo "  just push-gitea       # Push to Gitea only"
echo "  just push-all         # Push to both GitHub and Gitea"
echo "  just sync-gitea       # Sync Gitea with GitHub"
echo "  just remotes          # Show all remotes"
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
