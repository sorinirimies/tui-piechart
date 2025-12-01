# tui-piechart - A customizable pie chart widget for Ratatui
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

# Build examples in release mode
build-examples:
    cargo build --examples --release

# Run the main piechart example
run:
    cargo run --example piechart

# Run a specific example (usage: just run-example piechart)
run-example NAME:
    cargo run --example {{NAME}}

# Run the piechart example
run-piechart: (run-example "piechart")

# Run the custom_symbols example
run-custom-symbols: (run-example "custom_symbols")

# Run the high_resolution example
run-high-resolution: (run-example "high_resolution")

# Run the symbols_circles_squares example
run-symbols-circles-squares: (run-example "symbols_circles_squares")

# Run the symbols_stars_hearts example
run-symbols-stars-hearts: (run-example "symbols_stars_hearts")

# Run the symbols_triangles_hexagons example
run-symbols-triangles-hexagons: (run-example "symbols_triangles_hexagons")

# Run the symbols_shades_bars example
run-symbols-shades-bars: (run-example "symbols_shades_bars")

# Run the border_styles example
run-border-styles: (run-example "border_styles")

# Run the legend_positioning example
run-legend-positioning: (run-example "legend_positioning")

# Run the legend_alignment example
run-legend-alignment: (run-example "legend_alignment")

# Run the title_positioning example
run-title-positioning: (run-example "title_positioning")

# Run the title_styles_example example
run-title-styles-example: (run-example "title_styles_example")

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
bump version: check-all check-git-cliff
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
    cargo watch -x "run --example piechart"

# Git: commit current changes
commit message:
    git add .
    git commit -m "{{message}}"

# Git: pull from Gitea
pull-gitea:
    git pull gitea main

# Git: push to a specific remote
push-remote REMOTE BRANCH:
    git push {{REMOTE}} {{BRANCH}}

# Git: push to GitHub (origin)
push: (push-remote "origin" "main")

# Git: push to Gitea
push-gitea: (push-remote "gitea" "main")

# Git: push to both GitHub and Gitea
push-all:
    @just push-remote origin main
    @just push-remote gitea main
    @echo "✅ Pushed to both GitHub and Gitea!"

# Git: push tags to a specific remote
push-tags-remote REMOTE:
    git push {{REMOTE}} --tags

# Git: push tags to GitHub
push-tags: (push-tags-remote "origin")

# Git: push tags to both remotes
push-tags-all:
    @just push-tags-remote origin
    @just push-tags-remote gitea
    @echo "✅ Tags pushed to both GitHub and Gitea!"

# Full release workflow: bump version and push to a remote
release-to-remote version REMOTE:
    @just bump {{version}}
    @echo "Pushing to {{REMOTE}}..."
    @just push-remote {{REMOTE}} main
    git push {{REMOTE}} v{{version}}
    @echo "✅ Release v{{version}} complete on {{REMOTE}}!"

# Full release workflow: bump version and push to GitHub
release version: (release-to-remote version "origin")

# Full release workflow: bump version and push to Gitea
release-gitea version: (release-to-remote version "gitea")

# Full release workflow: bump version and push to both GitHub and Gitea
release-all version:
    @just bump {{version}}
    @echo "Pushing to both GitHub and Gitea..."
    @just push-remote origin main
    @just push-remote gitea main
    git push origin v{{version}}
    git push gitea v{{version}}
    @echo "✅ Release v{{version}} complete on both remotes!"

# Push release to both GitHub and Gitea (without bumping)
push-release-all:
    @echo "Pushing release to both GitHub and Gitea..."
    @just push-all
    @just push-tags-all
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
    @echo "Project: tui-piechart"
    @echo "Version: $(just version)"
    @echo "Author: Sorin Albu-Irimies"
    @echo "License: MIT"

# View changelog
view-changelog:
    @cat CHANGELOG.md

# Generic VHS tape runner (usage: just vhs-tape piechart)
vhs-tape NAME:
    @echo "Running VHS tape to generate {{NAME}} demo..."
    vhs examples/vhs/{{NAME}}.tape
    @echo "✅ Demo generated at examples/vhs/output/{{NAME}}.gif"

# Run the VHS tape to generate demo GIF for piechart
vhs-piechart: (vhs-tape "piechart")

# Run the VHS tape to generate demo GIF for custom_symbols
vhs-custom-symbols: (vhs-tape "custom_symbols")

# Run the VHS tape to generate demo GIF for high_resolution
vhs-high-resolution: (vhs-tape "high_resolution")

# Run the VHS tape to generate demo GIF for symbols_circles_squares
vhs-symbols-circles-squares: (vhs-tape "symbols_circles_squares")

# Run the VHS tape to generate demo GIF for symbols_stars_hearts
vhs-symbols-stars-hearts: (vhs-tape "symbols_stars_hearts")

# Run the VHS tape to generate demo GIF for symbols_triangles_hexagons
vhs-symbols-triangles-hexagons: (vhs-tape "symbols_triangles_hexagons")

# Run the VHS tape to generate demo GIF for symbols_shades_bars
vhs-symbols-shades-bars: (vhs-tape "symbols_shades_bars")

# Run the VHS tape to generate demo GIF for border_styles
vhs-border-styles: (vhs-tape "border_styles")

# Run the VHS tape to generate demo GIF for legend_positioning
vhs-legend-positioning: (vhs-tape "legend_positioning")

# Run the VHS tape to generate demo GIF for legend_alignment
vhs-legend-alignment: (vhs-tape "legend_alignment")

# Run the VHS tape to generate demo GIF for title_positioning
vhs-title-positioning: (vhs-tape "title_positioning")

# Run the VHS tape to generate demo GIF for title_styles_example
vhs-title-styles-example: (vhs-tape "title_styles")

# Run all VHS tapes to generate all demo GIFs
vhs-all: build-examples vhs-piechart vhs-custom-symbols vhs-high-resolution vhs-symbols-circles-squares vhs-symbols-stars-hearts vhs-symbols-triangles-hexagons vhs-symbols-shades-bars vhs-border-styles vhs-legend-positioning vhs-legend-alignment vhs-title-positioning vhs-title-styles-example
    @echo "✅ All demo GIFs generated!"

# ============================================
# Commands added by setup-just.sh
# ============================================

# Git: pull from GitHub (origin)
pull:
    git pull origin main


# Git: pull from both (Gitea first, then GitHub)
pull-all:
    git pull gitea main
    git pull origin main
    @echo "✅ Pulled from both Gitea and GitHub!"


