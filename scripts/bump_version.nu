#!/usr/bin/env nu
# Automated version bump script for tui-piechart
# Usage: nu scripts/bump_version.nu [--yes] <new_version>
# Example: nu scripts/bump_version.nu 0.3.2
#          nu scripts/bump_version.nu --yes 0.3.2   # skip confirmation
#
# Updates:
#   • [package] version in Cargo.toml
#   • Version badge in README.md
#   • Cargo.lock (cargo update -p tui-piechart)
#   • CHANGELOG.md (via git-cliff)
# Then commits and tags locally.  Use `just push-release-all` to push.

# ── Helpers ───────────────────────────────────────────────────────────────────

# Read the current [package] version from a Cargo.toml string.
export def read_package_version [content: string] {
    $content
    | lines
    | reduce --fold { in_pkg: false, version: "" } { |line, acc|
        let new_in_pkg = if ($line =~ '^\[package\]') {
            true
        } else if ($acc.in_pkg and ($line =~ '^\[')) {
            false
        } else {
            $acc.in_pkg
        }

        let new_version = if ($acc.in_pkg and ($line =~ '^version\s*=\s*"[^"]*"')) {
            $line
            | parse --regex 'version\s*=\s*"(?P<v>[^"]+)"'
            | get v
            | first
        } else {
            $acc.version
        }

        { in_pkg: $new_in_pkg, version: $new_version }
    }
    | get version
}

# Replace the version line inside [package] only.
export def update_package_version [lines: list<string>, new_version: string] {
    let result = $lines | reduce --fold { in_pkg: false, lines: [] } { |line, acc|
        let new_in_pkg = if ($line =~ '^\[package\]') {
            true
        } else if ($acc.in_pkg and ($line =~ '^\[')) {
            false
        } else {
            $acc.in_pkg
        }

        let new_line = if ($acc.in_pkg and ($line =~ '^version\s*=\s*"[^"]*"')) {
            $'version = "($new_version)"'
        } else {
            $line
        }

        { in_pkg: $new_in_pkg, lines: ($acc.lines | append $new_line) }
    }
    $result.lines
}

# ── Main ──────────────────────────────────────────────────────────────────────

def main [
    new_version: string,  # New version in X.Y.Z format
    --yes (-y),           # Skip confirmation prompt (non-interactive)
] {
    let red    = (ansi red)
    let green  = (ansi green)
    let yellow = (ansi yellow)
    let cyan   = (ansi cyan)
    let reset  = (ansi reset)

    # ── Validate version format ───────────────────────────────────────────────
    if not ($new_version =~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$') {
        error make { msg: $"($red)Error: Invalid version format($reset)
Version must be in format: X.Y.Z or X.Y.Z-suffix \(e.g., 0.3.2 or 1.0.0-beta.1\)" }
    }

    print $"($cyan)════════════════════════════════════════($reset)"
    print $"($cyan)  tui-piechart Version Bump($reset)"
    print $"($cyan)════════════════════════════════════════($reset)"
    print ""

    # ── Read current version from [package] ───────────────────────────────────
    let cargo_content = (open Cargo.toml --raw)
    let cargo_lines   = ($cargo_content | lines)

    let current_version = (read_package_version $cargo_content)

    if ($current_version | is-empty) {
        error make { msg: $"($red)Error: Could not read [package] version from Cargo.toml($reset)" }
    }

    print $"Current version: ($yellow)($current_version)($reset)"
    print $"New version:     ($green)($new_version)($reset)"
    print ""

    # ── Guard: already at requested version ──────────────────────────────────
    if $current_version == $new_version {
        error make { msg: $"($red)Error: Cargo.toml is already at version ($new_version).($reset)
($yellow)  Bump to the next version, or delete the tag if you need to re-release:($reset)
      git tag -d v($new_version) && git push origin :refs/tags/v($new_version)" }
    }

    # ── Guard: tag already exists locally ────────────────────────────────────
    let tag_name = $"v($new_version)"
    let existing_tags = (git tag | lines)
    if ($existing_tags | any { |t| $t == $tag_name }) {
        error make { msg: $"($red)Error: Tag ($tag_name) already exists locally.($reset)
($yellow)  Delete it first if you really want to recreate it:($reset)
      git tag -d ($tag_name)" }
    }

    # ── Confirmation ─────────────────────────────────────────────────────────
    if $yes {
        print $"($cyan)Running non-interactively \(--yes passed\).($reset)"
    } else {
        let reply = (input "Continue with version bump? (y/n) ")
        if not ($reply =~ '^[Yy]') {
            print $"($yellow)Aborted($reset)"
            return
        }
    }

    print ""

    # ── Step 1: Update [package] version ──────────────────────────────────────
    print $"($cyan)Step 1/6: Updating [package] version in Cargo.toml...($reset)"

    let final_lines = (update_package_version $cargo_lines $new_version)
    $final_lines | str join "\n" | save --force Cargo.toml

    # Verify version took effect
    let verify_version = (read_package_version (open Cargo.toml --raw))
    if $verify_version != $new_version {
        error make { msg: $"($red)Failed to update [package] version \(got ($verify_version)\).($reset)
($yellow)  Check the version line format in Cargo.toml and update manually.($reset)" }
    }
    print $"($green)✓ Cargo.toml updated \(($current_version) → ($new_version)\)($reset)"

    # ── Step 2: Update README.md badges + installation snippet ───────────────
    print ""
    print $"($cyan)Step 2/6: Updating README.md...($reset)"

    if ("README.md" | path exists) {
        let readme = (open README.md --raw)

        # Update version badge
        let readme = if ($readme =~ 'version-[0-9]+\.[0-9]+\.[0-9]+-blue') {
            let updated = (
                $readme
                | str replace --all --regex 'version-[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?-blue' $"version-($new_version)-blue"
            )
            print $"($green)✓ README.md badges updated($reset)"
            $updated
        } else {
            print $"($yellow)⚠ No version badge found in README.md — skipping($reset)"
            $readme
        }

        # Update installation snippet: tui-piechart = "X.Y.Z"
        let readme = if ($readme =~ 'tui-piechart\s*=\s*"[0-9]+\.[0-9]+\.[0-9]+"') {
            let updated = (
                $readme
                | str replace --all --regex 'tui-piechart\s*=\s*"[0-9]+\.[0-9]+\.[0-9]+"' $'tui-piechart = "($new_version)"'
            )
            print $"($green)✓ README.md installation snippet updated($reset)"
            $updated
        } else {
            print $"($yellow)⚠ No tui-piechart version pin found in README.md — skipping($reset)"
            $readme
        }

        # Update ratatui version from Cargo.toml
        let ratatui_ver = (
            $cargo_content
            | lines
            | where { |line| $line =~ 'ratatui\s*=' }
            | first
            | parse --regex 'ratatui\s*=\s*\{?\s*version\s*=\s*"(?P<v>[^"]+)"'
            | get v
            | first
        )
        let readme = if (not ($ratatui_ver | is-empty)) and ($readme =~ 'ratatui\s*=\s*"[0-9]+\.[0-9]+"') {
            let updated = (
                $readme
                | str replace --all --regex 'ratatui\s*=\s*"[0-9]+\.[0-9]+"' $'ratatui = "($ratatui_ver)"'
            )
            print $"($green)✓ README.md ratatui version updated to ($ratatui_ver)($reset)"
            $updated
        } else {
            $readme
        }

        $readme | save --force README.md
    } else {
        print $"($yellow)⚠ README.md not found — skipping($reset)"
    }

    # ── Step 3: Update Cargo.lock ─────────────────────────────────────────────
    print ""
    print $"($cyan)Step 3/6: Updating Cargo.lock...($reset)"
    run-external "cargo" "update" "-p" "tui-piechart"
    print $"($green)✓ Cargo.lock updated($reset)"

    # ── Step 4: cargo fmt ─────────────────────────────────────────────────────
    print ""
    print $"($cyan)Step 4/6: Running cargo fmt...($reset)"
    run-external "cargo" "fmt"
    print $"($green)✓ Code formatted($reset)"

    # ── Step 5: cargo clippy ──────────────────────────────────────────────────
    print ""
    print $"($cyan)Step 5/6: Running cargo clippy...($reset)"
    let clippy = (do {
        run-external "cargo" "clippy" "--" "-D" "warnings"
    } | complete)
    if $clippy.exit_code != 0 {
        error make { msg: $"($red)✗ Clippy found issues. Please fix them before continuing.($reset)" }
    }
    print $"($green)✓ Clippy passed($reset)"

    # ── Step 6: cargo test + CHANGELOG + commit + tag ─────────────────────────
    print ""
    print $"($cyan)Step 6/6: Running tests, generating changelog, committing...($reset)"

    let tests = (do {
        run-external "cargo" "test" "--locked" "--all-features" "--all-targets"
    } | complete)
    if $tests.exit_code != 0 {
        error make { msg: $"($red)✗ Tests failed. Please fix them before continuing.($reset)" }
    }
    print $"($green)✓ All tests passed($reset)"

    if (which git-cliff | length) > 0 {
        run-external "git-cliff" "--tag" $tag_name "-o" "CHANGELOG.md"
        print $"($green)✓ CHANGELOG.md generated($reset)"
    } else {
        print $"($yellow)⚠ git-cliff not found — skipping changelog generation($reset)"
        print $"($yellow)  Install it with: cargo install git-cliff($reset)"
    }

    # Stage changed files
    run-external "git" "add" "Cargo.toml" "Cargo.lock" "README.md" "CHANGELOG.md"
    let commit_msg = $"chore: bump version to ($new_version)

- Update version in Cargo.toml to ($new_version)
- Update version badge in README.md
- Regenerate Cargo.lock
- Generate updated CHANGELOG.md"
    run-external "git" "commit" "-m" $commit_msg

    let tag_msg = $"Release ($tag_name)

Includes all changes documented in CHANGELOG.md for version ($new_version)."
    run-external "git" "tag" "-a" $tag_name "-m" $tag_msg
    print $"($green)✓ Tag ($tag_name) created($reset)"

    # ── Summary ───────────────────────────────────────────────────────────────
    print ""
    print $"($cyan)════════════════════════════════════════($reset)"
    print $"($green)✓ Version bump complete! 🚀($reset)"
    print $"($cyan)════════════════════════════════════════($reset)"
    print ""
    print $"($yellow)Next steps:($reset)"
    print  "  1. Review the changes:"
    print $"     ($cyan)git show($reset)"
    print  ""
    print  "  2. Push to GitHub (triggers the release workflow):"
    print $"     ($cyan)git push --follow-tags origin main($reset)"
    print  ""
    print  "  3. Push to Gitea as well:"
    print $"     ($cyan)git push --follow-tags gitea main($reset)"
    print  ""
    print  "  4. Or use the just shortcuts:"
    print $"     ($cyan)just push-release-all($reset)   # push branch + tags to all remotes"
    print ""
}
