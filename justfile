# tui-piechart — A customizable pie chart widget for Ratatui
#
# Install just:      cargo install just
# Install git-cliff: cargo install git-cliff
# Install nushell:   https://www.nushell.sh
# Usage:             just <task>   |   just --list

# Default task — show available commands
default:
    @just --list

# ── Prerequisites ─────────────────────────────────────────────────────────────

# Install required tools (just, git-cliff). Nu must be installed manually.
install-tools:
    @echo "Installing required tools..."
    @command -v just >/dev/null 2>&1 || cargo install just
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff
    @command -v nu >/dev/null 2>&1 && echo "✅ nu found" || echo "⚠ nu (nushell) not found. Install: https://www.nushell.sh"
    @echo "✅ Done!"

# Check git-cliff is available
check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { echo "❌ git-cliff not found. Run: just install-tools"; exit 1; }

# Check nu (nushell) is available
check-nu:
    @command -v nu >/dev/null 2>&1 || { echo "❌ nu (nushell) not found. Install: https://www.nushell.sh"; exit 1; }

# ── Build ─────────────────────────────────────────────────────────────────────

# Build the project (debug)
build:
    cargo build

# Build release version
build-release:
    cargo build --release

# Build examples in release mode
build-examples:
    cargo build --examples --release

# ── Run examples ──────────────────────────────────────────────────────────────

# Run a specific example (usage: just run-example piechart)
run-example NAME:
    cargo run --example {{ NAME }}

# Run the main piechart example
run: (run-example "piechart")

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

# ── Code quality ──────────────────────────────────────────────────────────────

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Check formatting (CI-safe, no writes)
fmt-check:
    cargo fmt --check

# Run clippy linter
clippy:
    cargo clippy -- -D warnings

# Run all quality checks (fmt, clippy, test)
check-all: fmt-check clippy test
    @echo "✅ All checks passed!"

# ── Tests ─────────────────────────────────────────────────────────────────────

# Run tests
test:
    cargo test

# Run tests with all features
test-all:
    cargo test --all-features --all-targets

# Run tests with coverage
test-coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Run Nu script tests
test-nu: check-nu
    nu scripts/tests/run_all.nu

# Run both Rust and Nu tests
test-all-nu: test-all test-nu
    @echo "✅ All Rust and Nu tests passed!"

# ── Documentation ─────────────────────────────────────────────────────────────

# Build and open docs
doc:
    cargo doc --no-deps --open

# Build docs (no open, for CI)
doc-build:
    cargo doc --no-deps --all-features

# ── Changelog ─────────────────────────────────────────────────────────────────

# Generate full changelog from all tags
changelog: check-git-cliff
    @echo "Generating full changelog..."
    git-cliff -o CHANGELOG.md
    @echo "✅ Changelog generated!"

# Preview unreleased changes (no file write)
changelog-preview: check-git-cliff
    @git-cliff --unreleased

# Prepend unreleased commits to existing changelog
changelog-unreleased: check-git-cliff
    @echo "Prepending unreleased commits..."
    git-cliff --unreleased --prepend CHANGELOG.md
    @echo "✅ Done!"

# Generate changelog for a specific version tag
changelog-version version: check-git-cliff
    @echo "Generating changelog for version {{ version }}..."
    git-cliff --tag v{{ version }} -o CHANGELOG.md
    @echo "✅ Changelog generated for v{{ version }}!"

# Regenerate full changelog from all history
changelog-update: check-git-cliff
    @echo "Regenerating complete changelog..."
    git-cliff --output CHANGELOG.md
    @echo "✅ Changelog updated!"

# Preview unreleased changes only
changelog-preview-unreleased: check-git-cliff
    @git-cliff --unreleased

# Generate changelog for latest tag only
changelog-latest: check-git-cliff
    @echo "Generating changelog for latest tag..."
    git-cliff --latest -o CHANGELOG.md
    @echo "✅ Latest changelog generated!"

# ── Versioning & Release ──────────────────────────────────────────────────────

# Show current version
version: check-nu
    @nu scripts/version.nu

# Bump version (usage: just bump 0.2.0)
bump version: check-all check-git-cliff check-nu
    nu scripts/bump_version.nu {{ version }}

# Run pre-publish readiness checks
release-check: check-nu
    nu scripts/check_publish.nu

# Quick release: format, check, test, and build
release-quick-check: fmt clippy test build-release
    @echo "✅ Ready for release!"

# Publish to crates.io (dry run)
publish-dry:
    cargo publish --dry-run

# Publish to crates.io
publish: release-check
    cargo publish

# ── Full release workflows ────────────────────────────────────────────────────

# Full automated release to GitHub — bumps version, commits, tags, and pushes.
release version: check-all check-git-cliff check-nu
    nu scripts/bump_version.nu --yes {{ version }}
    @echo "Pushing branch and tag to GitHub..."
    git push origin main
    git push origin v{{ version }}
    @echo "✅ Release v{{ version }} pushed — GitHub Actions will handle the rest."

# Full automated release to Gitea only.
release-gitea version: check-all check-git-cliff check-nu
    nu scripts/bump_version.nu --yes {{ version }}
    @echo "Pushing branch and tag to Gitea..."
    git push gitea main
    git push gitea v{{ version }}
    @echo "✅ Release v{{ version }} pushed to Gitea."

# Full automated release to Gitea Starscream only.
release-gitea-starscream version: check-all check-git-cliff check-nu
    nu scripts/bump_version.nu --yes {{ version }}
    @echo "Pushing branch and tag to Gitea Starscream..."
    git push gitea_starscream main
    git push gitea_starscream v{{ version }}
    @echo "✅ Release v{{ version }} pushed to Gitea Starscream."

# Full automated release to all remotes (continues on failure).
release-all version: check-all check-git-cliff check-nu
    #!/usr/bin/env sh
    set -e
    nu scripts/bump_version.nu --yes {{ version }}
    set +e
    echo "Pushing release v{{ version }} to all remotes…"
    failed=""
    git push --follow-tags origin main             || failed="$failed origin"
    git push --follow-tags gitea main              || failed="$failed gitea"
    git push --follow-tags gitea_starscream main   || failed="$failed gitea_starscream"
    if [ -n "$failed" ]; then
        echo "⚠️  Release v{{ version }} failed to push to:$failed"
    else
        echo "✅ Release v{{ version }} pushed to GitHub, Gitea, and Gitea Starscream!"
    fi

# Push the latest commit and all tags to every remote (no bump, continues on failure).
push-release-all: check-all
    #!/usr/bin/env sh
    failed=""
    git push --follow-tags origin main             || failed="$failed origin"
    git push --follow-tags gitea main              || failed="$failed gitea"
    git push --follow-tags gitea_starscream main   || failed="$failed gitea_starscream"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to push to:$failed"
    else
        echo "✅ Latest commit + tags pushed to all remotes."
    fi

# ── Dependencies ──────────────────────────────────────────────────────────────

# Update dependencies (Cargo.lock only)
update:
    cargo update

# Update dependencies, run quality gate, commit and push to GitHub.

# Aborts without committing if fmt, clippy, or tests fail.
update-deps:
    @echo "⬆️  Updating dependencies…"
    cargo update
    @echo "🔍 Running quality gate…"
    cargo fmt -- --check
    cargo clippy -- -D warnings -A deprecated
    cargo test --locked --all-features --all-targets
    @echo "✅ All checks passed — committing dependency updates…"
    git add Cargo.lock
    git diff --cached --quiet || git commit -m "chore: update dependencies"
    git push origin main
    @echo "✅ Dependency updates pushed to GitHub."

# Update dependencies and push to Gitea
update-deps-gitea:
    @echo "⬆️  Updating dependencies…"
    cargo update
    @echo "🔍 Running quality gate…"
    cargo fmt -- --check
    cargo clippy -- -D warnings -A deprecated
    cargo test --locked --all-features --all-targets
    @echo "✅ All checks passed — committing dependency updates…"
    git add Cargo.lock
    git diff --cached --quiet || git commit -m "chore: update dependencies"
    git push gitea main
    @echo "✅ Dependency updates pushed to Gitea."

# Update dependencies and push to Gitea Starscream
update-deps-gitea-starscream:
    @echo "⬆️  Updating dependencies…"
    cargo update
    @echo "🔍 Running quality gate…"
    cargo fmt -- --check
    cargo clippy -- -D warnings -A deprecated
    cargo test --locked --all-features --all-targets
    @echo "✅ All checks passed — committing dependency updates…"
    git add Cargo.lock
    git diff --cached --quiet || git commit -m "chore: update dependencies"
    git push gitea_starscream main
    @echo "✅ Dependency updates pushed to Gitea Starscream."

# Update dependencies and push to all remotes (continues on failure)
update-deps-all:
    #!/usr/bin/env sh
    echo "⬆️  Updating dependencies…"
    cargo update
    echo "🔍 Running quality gate…"
    cargo fmt -- --check || exit 1
    cargo clippy -- -D warnings -A deprecated || exit 1
    cargo test --locked --all-features --all-targets || exit 1
    echo "✅ All checks passed — committing dependency updates…"
    git add Cargo.lock
    git diff --cached --quiet || git commit -m "chore: update dependencies"
    failed=""
    git push origin main             || failed="$failed origin"
    git push gitea main              || failed="$failed gitea"
    git push gitea_starscream main   || failed="$failed gitea_starscream"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to push to:$failed"
    else
        echo "✅ Dependency updates pushed to all remotes."
    fi

# Show outdated dependencies (requires cargo-outdated)
outdated:
    cargo outdated

# Upgrade dependencies locally (dry-run by default)
upgrade-deps *FLAGS: check-nu
    nu scripts/upgrade_deps.nu --dry-run {{ FLAGS }}

# ── VHS / Demo GIFs ──────────────────────────────────────────────────────────

# Run a single VHS tape by name (usage: just vhs-tape piechart)
vhs-tape NAME: build-examples
    @echo "Running VHS tape to generate {{ NAME }} demo..."
    @mkdir -p examples/vhs/output
    vhs examples/vhs/{{ NAME }}.tape
    @echo "✅ Demo generated at examples/vhs/output/{{ NAME }}.gif"

# Run all VHS tapes to generate all demo GIFs (dynamically discovers all .tape files)
vhs-all: build-examples
    #!/usr/bin/env sh
    set -eu
    command -v vhs >/dev/null 2>&1 || { echo "❌ VHS not found! Install: brew install vhs"; exit 1; }
    mkdir -p examples/vhs/output
    tapes=$(find examples/vhs -maxdepth 1 -name '*.tape' -type f | sort)
    total=$(echo "$tapes" | wc -l | tr -d ' ')
    if [ "$total" -eq 0 ]; then echo "⚠ No .tape files found in examples/vhs/"; exit 0; fi
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  TUI PieChart — VHS Demo Generator"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Found $total VHS tape(s)"
    echo ""
    current=0; succeeded=0; failed=0
    for tape in $tapes; do
        current=$((current + 1))
        name=$(basename "$tape" .tape)
        echo "[$current/$total] Generating ${name}.gif..."
        if vhs "$tape"; then
            echo "✓ Generated ${name}.gif"
            succeeded=$((succeeded + 1))
        else
            echo "⚠ Failed to generate ${name}.gif"
            failed=$((failed + 1))
        fi
    done
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "✅ Demo generation complete! $succeeded succeeded, $failed failed."
    if [ "$succeeded" -gt 0 ]; then
        echo "Generated GIFs:"
        ls -lh examples/vhs/output/*.gif 2>/dev/null | awk '{printf "  • %s (%s)\n", $9, $5}' || true
    fi
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# ── Git remotes & pushing ────────────────────────────────────────────────────

# Show current git remotes
remotes:
    @git remote -v

# Add a Gitea remote (usage: just setup-gitea https://gitea.example.com/user/repo.git)
setup-gitea url:
    git remote add gitea {{ url }}
    @echo "✅ Gitea remote added! Test with: git push gitea main"

# Commit all staged changes
commit message:
    git add .
    git commit -m "{{ message }}"

# Pull from GitHub
pull:
    git pull origin main

# Pull from Gitea
pull-gitea:
    git pull gitea main

# Pull from Gitea Starscream
pull-gitea-starscream:
    git pull gitea_starscream main

# Pull from all remotes (continues on failure)
pull-all:
    #!/usr/bin/env sh
    failed=""
    git pull origin main             || failed="$failed origin"
    git pull gitea main              || failed="$failed gitea"
    git pull gitea_starscream main   || failed="$failed gitea_starscream"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to pull from:$failed"
    else
        echo "✅ Pulled from GitHub, Gitea, and Gitea Starscream!"
    fi

# Push to GitHub
push:
    git push origin main

# Push to Gitea
push-gitea:
    git push gitea main

# Push to Gitea Starscream
push-gitea-starscream:
    git push gitea_starscream main

# Push to all remotes (continues on failure)
push-all:
    #!/usr/bin/env sh
    failed=""
    git push origin main             || failed="$failed origin"
    git push gitea main              || failed="$failed gitea"
    git push gitea_starscream main   || failed="$failed gitea_starscream"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to push to:$failed"
    else
        echo "✅ Pushed to GitHub, Gitea, and Gitea Starscream!"
    fi

# Force-push to all remotes
push-all-force:
    #!/usr/bin/env sh
    failed=""
    git push --force origin main             || failed="$failed origin"
    git push --force gitea main              || failed="$failed gitea"
    git push --force gitea_starscream main   || failed="$failed gitea_starscream"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to force-push to:$failed"
    else
        echo "✅ Force-pushed to GitHub, Gitea, and Gitea Starscream!"
    fi

# Push tags to GitHub
push-tags:
    git push origin --tags

# Push tags to all remotes (continues on failure)
push-tags-all:
    #!/usr/bin/env sh
    failed=""
    git push origin --tags             || failed="$failed origin"
    git push gitea --tags              || failed="$failed gitea"
    git push gitea_starscream --tags   || failed="$failed gitea_starscream"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to push tags to:$failed"
    else
        echo "✅ Tags pushed to all remotes!"
    fi

# Force-sync Gitea from GitHub
sync-gitea:
    git push gitea main --force
    git push gitea --tags --force
    @echo "✅ Gitea synced!"

# Force-sync Gitea Starscream from GitHub
sync-gitea-starscream:
    git push gitea_starscream main --force
    git push gitea_starscream --tags --force
    @echo "✅ Gitea Starscream synced!"

# Force-sync all Gitea instances from GitHub (continues on failure)
sync-all-gitea:
    #!/usr/bin/env sh
    failed=""
    git push gitea main --force                  || failed="$failed gitea"
    git push gitea --tags --force                || failed="$failed gitea-tags"
    git push gitea_starscream main --force       || failed="$failed gitea_starscream"
    git push gitea_starscream --tags --force     || failed="$failed gitea_starscream-tags"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to sync:$failed"
    else
        echo "✅ All Gitea instances force-synced with GitHub."
    fi

# ── Misc ─────────────────────────────────────────────────────────────────────

# Clean build artifacts
clean:
    cargo clean

# Watch and auto-run on file changes (requires cargo-watch)
watch:
    cargo watch -x "run --example piechart"

# Show git-cliff info
cliff-info:
    @echo "Git-cliff configuration:"
    @echo "  Config file: cliff.toml"
    @echo "  Installed: $(command -v git-cliff >/dev/null 2>&1 && echo '✅ Yes' || echo '❌ No (run: just install-tools)')"
    @command -v git-cliff >/dev/null 2>&1 && git-cliff --version || true

# Show project info
info:
    @echo "Project:  tui-piechart"
    @echo "Version:  $(just version)"
    @echo "Author:   Sorin Albu-Irimies"
    @echo "License:  MIT"
    @echo "Crate:    https://crates.io/crates/tui-piechart"

# View changelog
view-changelog:
    @cat CHANGELOG.md

# Check publish readiness
check-publish: check-nu
    nu scripts/check_publish.nu
