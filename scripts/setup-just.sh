#!/usr/bin/env bash
# Setup script for installing and configuring just command runner
# This is a general-purpose script that works with any project
#
# Features:
#   - Installs just via cargo or system package managers (apt, brew, pacman, dnf)
#   - Creates a new justfile if one doesn't exist (interactive)
#   - Enhances existing justfile with missing common commands (optional)
#   - Detects and installs optional tools (git-cliff, VHS, cargo-watch)
#   - Sets up shell completion (bash, zsh, fish, PowerShell)
#   - Creates backups before modifying files
#   - Implements "fail early" pattern for version bumping
#
# Usage: ./scripts/setup-just.sh
#
# Examples:
#   ./scripts/setup-just.sh              # Interactive setup
#   just setup-just                      # If you already have just installed

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

header() {
    echo -e "${CYAN}════════════════════════════════════════${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}════════════════════════════════════════${NC}"
    echo ""
}

# Detect project name from current directory
PROJECT_NAME=$(basename "$(pwd)")

header "Just Command Runner Setup"
info "Project: $PROJECT_NAME"
echo ""

# Check if justfile exists
JUSTFILE_EXISTS=false
if [ -f "justfile" ]; then
    JUSTFILE_EXISTS=true
    success "Found existing justfile"
    echo ""
    read -p "Do you want to add missing commands to your justfile? (Y/n) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Nn]$ ]]; then
        info "Skipping justfile enhancement. Will only install just."
        SKIP_JUSTFILE_ENHANCEMENT=true
    fi
else
    warning "No justfile found in current directory"
    echo ""
    read -p "Do you want to create a new justfile with common commands? (Y/n) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Nn]$ ]]; then
        info "Creating new justfile..."
        CREATE_NEW_JUSTFILE=true
    else
        info "Skipping justfile creation. Will only install just."
        SKIP_JUSTFILE_ENHANCEMENT=true
    fi
fi
echo ""

# Check if just is already installed
if command -v just &> /dev/null; then
    JUST_VERSION=$(just --version | cut -d' ' -f2)
    success "just is already installed (version: $JUST_VERSION)"
    echo ""

    read -p "Do you want to reinstall/update just? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        info "Skipping just installation"
        SKIP_INSTALL=true
    fi
    echo ""
fi

# Install just if needed
if [ "$SKIP_INSTALL" != true ]; then
    info "Installing just command runner..."
    echo ""

    # Check if cargo is available
    if command -v cargo &> /dev/null; then
        info "Using cargo to install just..."
        cargo install just
        success "just installed via cargo"
    else
        # Try package managers
        if command -v apt-get &> /dev/null; then
            warning "cargo not found. Attempting to install via apt..."
            sudo apt-get update && sudo apt-get install -y just
        elif command -v brew &> /dev/null; then
            warning "cargo not found. Attempting to install via homebrew..."
            brew install just
        elif command -v pacman &> /dev/null; then
            warning "cargo not found. Attempting to install via pacman..."
            sudo pacman -S just
        elif command -v dnf &> /dev/null; then
            warning "cargo not found. Attempting to install via dnf..."
            sudo dnf install just
        else
            error "Could not install just automatically.

Installation options:
1. Install Rust and cargo first:
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   Then run: cargo install just

2. Use your system package manager (see: https://just.systems/man/en/packages.html)

3. Download pre-built binary from: https://github.com/casey/just/releases"
        fi

        success "just installed via system package manager"
    fi
    echo ""
fi

# Verify installation
if ! command -v just &> /dev/null; then
    error "just installation failed. Please install manually."
fi

JUST_VERSION=$(just --version | cut -d' ' -f2)
success "just version $JUST_VERSION is ready"
echo ""

# Check for optional tools based on justfile content
if grep -q "git-cliff" justfile 2>/dev/null; then
    info "Checking for git-cliff (changelog generator)..."
    if command -v git-cliff &> /dev/null; then
        GIT_CLIFF_VERSION=$(git-cliff --version | cut -d' ' -f2)
        success "git-cliff is already installed (version: $GIT_CLIFF_VERSION)"
    else
        warning "git-cliff not found (needed for changelog commands)"
        echo ""
        read -p "Do you want to install git-cliff? (Y/n) " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Nn]$ ]]; then
            if command -v cargo &> /dev/null; then
                info "Installing git-cliff..."
                cargo install git-cliff
                success "git-cliff installed"
            else
                warning "cargo not found. Skipping git-cliff installation."
                info "Install it later with: cargo install git-cliff"
            fi
        fi
    fi
    echo ""
fi

if grep -q "vhs" justfile 2>/dev/null; then
    info "Checking for VHS (terminal recorder)..."
    if command -v vhs &> /dev/null; then
        success "VHS is installed"
    else
        warning "VHS not found (needed for demo/recording commands)"
        info "Install VHS: https://github.com/charmbracelet/vhs"
    fi
    echo ""
fi

if grep -q "cargo-watch" justfile 2>/dev/null || grep -q "watch" justfile 2>/dev/null; then
    info "Checking for cargo-watch (auto-rebuild tool)..."
    if cargo watch --version &> /dev/null; then
        success "cargo-watch is installed"
    else
        warning "cargo-watch not found (needed for watch commands)"
        info "Install it with: cargo install cargo-watch"
    fi
    echo ""
fi

# Display justfile commands
if [ "$JUSTFILE_EXISTS" = true ] || [ "$CREATE_NEW_JUSTFILE" = true ]; then
    header "Available Commands"
    just --list
    echo ""
fi

# Show quick start based on common command patterns
header "Quick Start Guide"
echo ""

if grep -q "^build:" justfile 2>/dev/null; then
    info "Build commands:"
    grep "^[a-z-]*build[a-z-]*:" justfile | sed 's/:.*//; s/^/  just /' || true
    echo ""
fi

if grep -q "^test:" justfile 2>/dev/null || grep -q "^check" justfile 2>/dev/null; then
    info "Testing & checking:"
    grep -E "^(test|check)[a-z-]*:" justfile | sed 's/:.*//; s/^/  just /' || true
    echo ""
fi

if grep -q "^run:" justfile 2>/dev/null || grep -q "^example" justfile 2>/dev/null; then
    info "Running code:"
    grep -E "^(run|example)[a-z-]*:" justfile | head -5 | sed 's/:.*//; s/^/  just /' || true
    echo ""
fi

if grep -q "^push" justfile 2>/dev/null || grep -q "^pull" justfile 2>/dev/null; then
    info "Git operations:"
    grep -E "^(push|pull)[a-z-]*:" justfile | sed 's/:.*//; s/^/  just /' || true
    echo ""
fi

if grep -q "^release" justfile 2>/dev/null || grep -q "^bump" justfile 2>/dev/null; then
    info "Release management:"
    grep -E "^(release|bump)[a-z-]*:" justfile | sed 's/:.*//; s/^/  just /' || true
    echo ""
fi

# Completion setup
header "Shell Completion (Optional)"
echo ""
info "just supports shell completion for bash, zsh, fish, and PowerShell"
echo ""
echo "To set up completion for your shell:"
echo ""
echo "  ${CYAN}Bash:${NC}"
echo "    mkdir -p ~/.local/share/bash-completion/completions"
echo "    just --completions bash > ~/.local/share/bash-completion/completions/just"
echo ""
echo "  ${CYAN}Zsh:${NC}"
echo "    mkdir -p ~/.zsh/completion"
echo "    just --completions zsh > ~/.zsh/completion/_just"
echo "    # Add to ~/.zshrc: fpath=(~/.zsh/completion \$fpath)"
echo ""
echo "  ${CYAN}Fish:${NC}"
echo "    just --completions fish > ~/.config/fish/completions/just.fish"
echo ""
echo "  ${CYAN}PowerShell:${NC}"
echo "    just --completions powershell > (Join-Path \$PROFILE.CurrentUserAllHosts '..' Completions just-completion.ps1)"
echo ""

read -p "Do you want to set up shell completion now? (y/N) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Detect shell
    SHELL_NAME=$(basename "$SHELL")

    case "$SHELL_NAME" in
        bash)
            COMP_DIR="$HOME/.local/share/bash-completion/completions"
            mkdir -p "$COMP_DIR"
            just --completions bash > "$COMP_DIR/just"
            success "Bash completion installed"
            info "Restart your terminal or run: source ~/.bashrc"
            ;;
        zsh)
            COMP_DIR="$HOME/.zsh/completion"
            mkdir -p "$COMP_DIR"
            just --completions zsh > "$COMP_DIR/_just"
            success "Zsh completion installed to $COMP_DIR/_just"

            # Check if fpath is configured
            if ! grep -q "fpath=.*\.zsh/completion" ~/.zshrc 2>/dev/null; then
                warning "You need to add $COMP_DIR to your \$fpath"
                echo ""
                read -p "Add to ~/.zshrc automatically? (Y/n) " -n 1 -r
                echo ""
                if [[ ! $REPLY =~ ^[Nn]$ ]]; then
                    echo "" >> ~/.zshrc
                    echo "# just completion" >> ~/.zshrc
                    echo "fpath=(~/.zsh/completion \$fpath)" >> ~/.zshrc
                    success "Added to ~/.zshrc"
                fi
            fi
            info "Restart your terminal or run: source ~/.zshrc"
            ;;
        fish)
            COMP_DIR="$HOME/.config/fish/completions"
            mkdir -p "$COMP_DIR"
            just --completions fish > "$COMP_DIR/just.fish"
            success "Fish completion installed"
            info "Restart your terminal"
            ;;
        *)
            warning "Unknown shell: $SHELL_NAME"
            info "Manually run: just --completions <shell> > <completion-file>"
            info "Supported shells: bash, zsh, fish, powershell"
            ;;
    esac
fi
echo ""

# Create new justfile if requested
if [ "$CREATE_NEW_JUSTFILE" = true ]; then
    header "Creating New Justfile"
    echo ""

    # Detect project type
    IS_RUST_PROJECT=false
    if [ -f "Cargo.toml" ]; then
        IS_RUST_PROJECT=true
        PROJECT_NAME_FROM_TOML=$(grep '^name = ' Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/' 2>/dev/null || echo "$PROJECT_NAME")
        info "Detected Rust project: $PROJECT_NAME_FROM_TOML"
    else
        info "Creating generic justfile"
    fi
    echo ""

    # Create the justfile
    cat > justfile << 'EOF'
# Justfile for project automation
#
# Setup: Run './scripts/setup-just.sh' for interactive installation
# Or install manually: cargo install just
# Usage: just <task> or just --list
# Patterns: See https://just.systems/man/en/ for documentation

# Default task - show available commands
default:
    @just --list

EOF

    if [ "$IS_RUST_PROJECT" = true ]; then
        cat >> justfile << 'EOF'
# Build the project
build:
    cargo build

# Build release version
build-release:
    cargo build --release

# Run tests
test:
    cargo test

# Format code
fmt:
    cargo fmt

# Check if code is formatted
fmt-check:
    cargo fmt --check

# Run clippy linter
clippy:
    cargo clippy -- -D warnings

# Run all checks (fmt-check, clippy, test) - fail early pattern
check-all: fmt-check clippy test
    @echo "✅ All checks passed!"

# Clean build artifacts
clean:
    cargo clean

# Show current version
version:
    @grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'

# Show project information
info:
    @echo "Project: $(grep '^name = ' Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/')"
    @grep '^version = ' Cargo.toml | head -1 | sed 's/version = /Version: /' | tr -d '"'
    @grep '^description = ' Cargo.toml | head -1 | sed 's/description = /Description: /' | tr -d '"' 2>/dev/null || true

# Install common development tools
install-tools:
    @echo "Installing development tools..."
    @command -v just >/dev/null 2>&1 || cargo install just
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff
    @echo "✅ Tools installed!"

# Check if git-cliff is installed
check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { echo "❌ git-cliff not found. Install with: cargo install git-cliff"; exit 1; }

# Bump version (usage: just bump 0.2.0) - runs all checks first
bump version: check-all check-git-cliff
    @echo "Bumping version to {{version}}..."
    @sed -i 's/^version = ".*"/version = "{{version}}"/' Cargo.toml
    @cargo update -p $(grep '^name = ' Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/')
    @git add Cargo.toml Cargo.lock
    @git commit -m "chore: bump version to {{version}}"
    @git tag -a "v{{version}}" -m "Release v{{version}}"
    @echo "✅ Version bumped to {{version}}"
    @echo "Next: git push origin main && git push origin v{{version}}"

# Generate documentation
doc:
    cargo doc --no-deps --open
EOF
    else
        cat >> justfile << 'EOF'
# Install common development tools
install-tools:
    @echo "Installing development tools..."
    @command -v just >/dev/null 2>&1 || cargo install just
    @echo "✅ Tools installed!"

# Show project information
info:
    @echo "Project: $(basename $(pwd))"
EOF
    fi

    success "Created justfile with common commands"
    info "Review the file: ${CYAN}cat justfile${NC}"
    info "Customize it for your project's needs"
    JUSTFILE_EXISTS=true
    echo ""
fi

# Offer to add common useful commands to existing justfile
if [ "$SKIP_JUSTFILE_ENHANCEMENT" != true ] && [ "$JUSTFILE_EXISTS" = true ] && [ "$CREATE_NEW_JUSTFILE" != true ]; then
    header "Enhance Justfile (Optional)"
    echo ""
    info "Checking for commonly useful commands that might be missing..."
    echo ""

# Check what's already in the justfile
HAS_DEFAULT=$(grep -q "^default:" justfile && echo "yes" || echo "no")
HAS_INSTALL_TOOLS=$(grep -q "^install-tools:" justfile && echo "yes" || echo "no")
HAS_CLEAN=$(grep -q "^clean:" justfile && echo "yes" || echo "no")
HAS_INFO=$(grep -q "^info:" justfile && echo "yes" || echo "no")
HAS_VERSION=$(grep -q "^version:" justfile && echo "yes" || echo "no")
HAS_BUILD=$(grep -q "^build:" justfile && echo "yes" || echo "no")
HAS_TEST=$(grep -q "^test:" justfile && echo "yes" || echo "no")
HAS_FMT=$(grep -q "^fmt:" justfile && echo "yes" || echo "no")
HAS_FMT_CHECK=$(grep -q "^fmt-check:" justfile && echo "yes" || echo "no")
HAS_CLIPPY=$(grep -q "^clippy:" justfile && echo "yes" || echo "no")
HAS_CHECK_ALL=$(grep -q "^check-all:" justfile && echo "yes" || echo "no")
HAS_BUMP=$(grep -q "^bump" justfile && echo "yes" || echo "no")

# Build list of missing commands
MISSING_COMMANDS=""
[ "$HAS_DEFAULT" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • default (show list of commands)"
[ "$HAS_INSTALL_TOOLS" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • install-tools (install common dev tools)"
[ "$HAS_BUILD" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • build (build the project)"
[ "$HAS_TEST" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • test (run tests)"
[ "$HAS_FMT" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • fmt (format code)"
[ "$HAS_FMT_CHECK" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • fmt-check (check code formatting)"
[ "$HAS_CLIPPY" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • clippy (lint code)"
[ "$HAS_CHECK_ALL" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • check-all (run all checks)"
[ "$HAS_CLEAN" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • clean (remove build artifacts)"
[ "$HAS_VERSION" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • version (display current version)"
[ "$HAS_INFO" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • info (show project information)"
[ "$HAS_BUMP" = "no" ] && MISSING_COMMANDS="${MISSING_COMMANDS}\n  • bump (bump version with checks)"

if [ -n "$MISSING_COMMANDS" ]; then
    info "Missing common commands:${MISSING_COMMANDS}"
    echo ""
    read -p "Add these commands to justfile? (y/N) " -n 1 -r
    echo ""

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Backup justfile
        cp justfile justfile.backup
        success "Created backup: justfile.backup"

        # Add missing commands
        ADDITIONS=""

        if [ "$HAS_DEFAULT" = "no" ]; then
            ADDITIONS="${ADDITIONS}
# Default task - show available commands
default:
    @just --list

"
        fi

        if [ "$HAS_INSTALL_TOOLS" = "no" ]; then
            ADDITIONS="${ADDITIONS}
# Install common development tools
install-tools:
    @echo \"Installing development tools...\"
    @command -v just >/dev/null 2>&1 || cargo install just
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff
    @echo \"✅ Tools installed!\"

"
        fi

        # Add Rust project commands only if Cargo.toml exists
        if [ -f "Cargo.toml" ]; then
            if [ "$HAS_BUILD" = "no" ]; then
                ADDITIONS="${ADDITIONS}
# Build the project
build:
    cargo build

"
            fi

            if [ "$HAS_TEST" = "no" ]; then
                ADDITIONS="${ADDITIONS}
# Run tests
test:
    cargo test

"
            fi

            if [ "$HAS_FMT" = "no" ]; then
                ADDITIONS="${ADDITIONS}
# Format code
fmt:
    cargo fmt

"
            fi

            if [ "$HAS_FMT_CHECK" = "no" ]; then
                ADDITIONS="${ADDITIONS}
# Check if code is formatted
fmt-check:
    cargo fmt --check

"
            fi

            if [ "$HAS_CLIPPY" = "no" ]; then
                ADDITIONS="${ADDITIONS}
# Run clippy linter
clippy:
    cargo clippy -- -D warnings

"
            fi

            if [ "$HAS_CHECK_ALL" = "no" ]; then
                ADDITIONS="${ADDITIONS}
# Run all checks (fmt-check, clippy, test) - fail early pattern
check-all: fmt-check clippy test
    @echo \"✅ All checks passed!\"

"
            fi

            if [ "$HAS_CLEAN" = "no" ]; then
                ADDITIONS="${ADDITIONS}
# Clean build artifacts
clean:
    cargo clean

"
            fi

            if [ "$HAS_VERSION" = "no" ]; then
                ADDITIONS="${ADDITIONS}
# Show current version
version:
    @grep '^version = ' Cargo.toml | head -1 | sed 's/version = \"\(.*\)\"/\1/'

"
            fi

            if [ "$HAS_INFO" = "no" ]; then
                PROJECT_NAME_FROM_TOML=$(grep '^name = ' Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/' 2>/dev/null || echo "$PROJECT_NAME")
                ADDITIONS="${ADDITIONS}
# Show project information
info:
    @echo \"Project: $PROJECT_NAME_FROM_TOML\"
    @grep '^version = ' Cargo.toml | head -1 | sed 's/version = /Version: /' | tr -d '\"'
    @grep '^description = ' Cargo.toml | head -1 | sed 's/description = /Description: /' | tr -d '\"' 2>/dev/null || true

"
            fi

            if [ "$HAS_BUMP" = "no" ]; then
                # Check if we have a bump script
                if [ -f "scripts/bump_version.sh" ]; then
                    ADDITIONS="${ADDITIONS}
# Check if git-cliff is installed
check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { echo \"❌ git-cliff not found. Install with: cargo install git-cliff\"; exit 1; }

# Bump version (usage: just bump 0.2.0) - runs all checks first
bump version: check-all check-git-cliff
    @echo \"Bumping version to {{version}}...\"
    @./scripts/bump_version.sh {{version}}

"
                else
                    ADDITIONS="${ADDITIONS}
# Bump version in Cargo.toml (usage: just bump 0.2.0) - runs all checks first
bump version: check-all
    @echo \"Bumping version to {{version}}...\"
    @sed -i 's/^version = \".*\"/version = \"{{version}}\"/' Cargo.toml
    @cargo update -p $(grep '^name = ' Cargo.toml | head -1 | sed 's/name = \"\(.*\)\"/\1/')
    @git add Cargo.toml Cargo.lock
    @git commit -m \"chore: bump version to {{version}}\"
    @git tag -a \"v{{version}}\" -m \"Release v{{version}}\"
    @echo \"✅ Version bumped to {{version}}\"

"
                fi
            fi
        fi

        if [ -n "$ADDITIONS" ]; then
            # Add separator comment
            echo "" >> justfile
            echo "# ============================================" >> justfile
            echo "# Commands added by setup-just.sh" >> justfile
            echo "# ============================================" >> justfile
            echo "$ADDITIONS" >> justfile

            success "Added useful commands to justfile"
            info "Review changes: ${CYAN}diff justfile.backup justfile${NC}"
            info "If needed, restore: ${CYAN}mv justfile.backup justfile${NC}"
        fi
    fi
else
    success "All common commands already present in justfile"
fi
echo ""
fi

# Show justfile location and documentation
if [ "$JUSTFILE_EXISTS" = true ] || [ "$CREATE_NEW_JUSTFILE" = true ]; then
    header "Documentation"
    echo ""
    info "Justfile location: $(pwd)/justfile"
    info "View all commands: ${CYAN}just --list${NC} or ${CYAN}just${NC}"
    info "View justfile source: ${CYAN}cat justfile${NC}"
    info "Edit justfile: ${CYAN}\$EDITOR justfile${NC}"
    echo ""
    info "Official documentation:"
    echo "  • just manual: https://just.systems/man/en/"
    echo "  • GitHub repo: https://github.com/casey/just"
    echo ""
fi

# Final summary
header "Setup Complete!"
success "just command runner is ready to use!"
echo ""
info "Quick reference:"
echo "  • List commands:     ${CYAN}just${NC} or ${CYAN}just --list${NC}"
echo "  • Run a command:     ${CYAN}just <command>${NC}"
echo "  • Show justfile:     ${CYAN}just --show <command>${NC}"
echo "  • Get help:          ${CYAN}just --help${NC}"
echo ""
info "Common usage patterns:"
echo "  • Build project:     ${CYAN}just build${NC}"
echo "  • Run tests:         ${CYAN}just test${NC}"
echo "  • Format code:       ${CYAN}just fmt${NC}"
echo "  • Run all checks:    ${CYAN}just check-all${NC}"
echo ""
success "Happy coding! 🚀"
