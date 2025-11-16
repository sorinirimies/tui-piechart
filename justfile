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

# Run the example
run:
    cargo run --example piechart

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
    cargo watch -x "run --example piechart"

# Git: commit current changes
commit message:
    git add .
    git commit -m "{{message}}"

pull-gitea:
    git pull gitea main

# Git: push to GitHub (origin)
push:
    git push origin main

# Git: push to Gitea
push-gitea:
    git push gitea main

# Git: push to both GitHub and Gitea
push-all:
    git push origin main
    git push gitea main
    @echo "✅ Pushed to both GitHub and Gitea!"

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
    @echo "Project: tui-piechart"
    @echo "Version: $(just version)"
    @echo "Author: Sorin Albu-Irimies"
    @echo "License: MIT"

# View changelog
view-changelog:
    @cat CHANGELOG.md

# Run the VHS tape to generate demo GIF for piechart
vhs-piechart:
    @echo "Running VHS tape to generate piechart demo..."
    vhs examples/vhs/piechart.tape
    @echo "✅ Demo generated at examples/piechart.gif"

# Run the VHS tape to generate demo GIF for custom_symbols
vhs-custom-symbols:
    @echo "Running VHS tape to generate custom_symbols demo..."
    vhs examples/vhs/custom_symbols.tape
    @echo "✅ Demo generated at examples/custom_symbols.gif"

# Run the VHS tape to generate demo GIF for high_resolution
vhs-high-resolution:
    @echo "Running VHS tape to generate high_resolution demo..."
    vhs examples/vhs/high_resolution.tape
    @echo "✅ Demo generated at examples/high_resolution.gif"

# Run the VHS tape to generate demo GIF for symbols_circles_squares
vhs-symbols-circles-squares:
    @echo "Running VHS tape to generate symbols_circles_squares demo..."
    vhs examples/vhs/symbols_circles_squares.tape
    @echo "✅ Demo generated at examples/symbols_circles_squares.gif"

# Run the VHS tape to generate demo GIF for symbols_stars_hearts
vhs-symbols-stars-hearts:
    @echo "Running VHS tape to generate symbols_stars_hearts demo..."
    vhs examples/vhs/symbols_stars_hearts.tape
    @echo "✅ Demo generated at examples/symbols_stars_hearts.gif"

# Run the VHS tape to generate demo GIF for symbols_triangles_hexagons
vhs-symbols-triangles-hexagons:
    @echo "Running VHS tape to generate symbols_triangles_hexagons demo..."
    vhs examples/vhs/symbols_triangles_hexagons.tape
    @echo "✅ Demo generated at examples/symbols_triangles_hexagons.gif"

# Run the VHS tape to generate demo GIF for symbols_shades_bars
vhs-symbols-shades-bars:
    @echo "Running VHS tape to generate symbols_shades_bars demo..."
    vhs examples/vhs/symbols_shades_bars.tape
    @echo "✅ Demo generated at examples/symbols_shades_bars.gif"

# Run all VHS tapes to generate all demo GIFs
vhs-all: vhs-piechart vhs-custom-symbols vhs-high-resolution vhs-symbols-circles-squares vhs-symbols-stars-hearts vhs-symbols-triangles-hexagons vhs-symbols-shades-bars
    @echo "✅ All demo GIFs generated!"
