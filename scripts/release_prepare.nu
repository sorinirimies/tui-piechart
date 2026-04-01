#!/usr/bin/env nu
# Prepare a release: update Cargo.toml version, regenerate CHANGELOG.md,
# and write RELEASE_NOTES.md for the GitHub/Gitea release body.
#
# Usage: nu scripts/release_prepare.nu <tag>
# Example: nu scripts/release_prepare.nu v0.3.2

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

# Build the RELEASE_NOTES.md content for tui-piechart.
export def build_release_notes [version: string, cliff_changes: string, last_tag: string] {
    let changes_header = if ($last_tag | is-empty) {
        "### 🎉 Initial Release"
    } else {
        $"### 📝 Changes since ($last_tag):"
    }

    [
        $"# tui-piechart ($version)"
        ""
        "## 🚀 What's New"
        ""
        $changes_header
        ""
        $cliff_changes
        ""
        "## 📦 Installation"
        ""
        "Add this to your `Cargo.toml`:"
        ""
        "```toml"
        "[dependencies]"
        $"tui-piechart = \"($version)\""
        "```"
        ""
        "Or install with cargo:"
        ""
        "```bash"
        "cargo add tui-piechart"
        "```"
        ""
        "## 🚀 Quick Start"
        ""
        "```rust"
        "use ratatui::style::Color;"
        "use tui_piechart::{PieChart, PieSlice};"
        ""
        "let slices = vec!["
        "    PieSlice::new(\"Rust\", 45.0, Color::Red),"
        "    PieSlice::new(\"Go\", 30.0, Color::Blue),"
        "];"
        "let piechart = PieChart::new(slices);"
        "```"
    ] | str join "\n"
}

# ── Main ──────────────────────────────────────────────────────────────────────

def main [
    tag: string,  # The release tag, e.g. "v0.3.2"
] {
    let green  = (ansi green)
    let cyan   = (ansi cyan)
    let red    = (ansi red)
    let yellow = (ansi yellow)
    let reset  = (ansi reset)

    # Strip leading 'v' to get the bare version number
    let version = ($tag | str replace --regex '^v' '')

    print $"($cyan)════════════════════════════════════════($reset)"
    print $"($cyan)  tui-piechart Release Prepare: ($tag)($reset)"
    print $"($cyan)════════════════════════════════════════($reset)"
    print ""

    # ── Step 1: Update Cargo.toml ─────────────────────────────────────────────
    print $"($cyan)Step 1/5: Updating Cargo.toml to ($version)...($reset)"

    let cargo_content = open Cargo.toml --raw
    let cargo_lines   = ($cargo_content | lines)

    let final_lines = update_package_version $cargo_lines $version
    $final_lines | str join "\n" | save --force Cargo.toml

    # Verify
    let got = read_package_version (open Cargo.toml --raw)
    if $got != $version {
        error make { msg: $"($red)Failed to update Cargo.toml \(got ($got), expected ($version)\)($reset)" }
    }
    print $"($green)✓ Cargo.toml updated to ($version)($reset)"

    # ── Step 2: Regenerate CHANGELOG.md ──────────────────────────────────────
    print ""
    print $"($cyan)Step 2/5: Regenerating CHANGELOG.md...($reset)"
    run-external "git-cliff" "--config" "cliff.toml" "--latest" "--output" "CHANGELOG.md"
    print $"($green)✓ CHANGELOG.md written($reset)"

    # ── Step 3: Generate per-release diff (CLIFF_CHANGES.md) ─────────────────
    print ""
    print $"($cyan)Step 3/5: Generating release diff...($reset)"

    let last_tag_result = (do { run-external "git" "describe" "--tags" "--abbrev=0" "HEAD^" } | complete)
    let last_tag = if $last_tag_result.exit_code == 0 {
        $last_tag_result.stdout | str trim
    } else {
        ""
    }

    if ($last_tag | is-empty) {
        print $"($yellow)  No previous tag found — generating full history for ($tag)($reset)"
        run-external "git-cliff" "--config" "cliff.toml" "--tag" $tag "--strip" "header" "--output" "CLIFF_CHANGES.md"
    } else {
        print $"  Diff range: ($last_tag)..($tag)"
        run-external "git-cliff" "--config" "cliff.toml" $"($last_tag)..($tag)" "--strip" "header" "--output" "CLIFF_CHANGES.md"
    }
    print $"($green)✓ CLIFF_CHANGES.md written($reset)"

    # ── Step 4: Build RELEASE_NOTES.md ────────────────────────────────────────
    print ""
    print $"($cyan)Step 4/5: Writing RELEASE_NOTES.md...($reset)"

    let cliff_changes = open CLIFF_CHANGES.md --raw
    let notes = build_release_notes $version $cliff_changes $last_tag
    $notes | save --force RELEASE_NOTES.md
    print $"($green)✓ RELEASE_NOTES.md written($reset)"

    # ── Step 5: Clean up temp file ────────────────────────────────────────────
    print ""
    print $"($cyan)Step 5/5: Cleaning up...($reset)"
    if ("CLIFF_CHANGES.md" | path exists) {
        rm CLIFF_CHANGES.md
    }
    print $"($green)✓ Temp files removed($reset)"

    # ── Summary ───────────────────────────────────────────────────────────────
    print ""
    print $"($cyan)════════════════════════════════════════($reset)"
    print $"($green)✓ Release ($tag) prepared successfully! 🚀($reset)"
    print $"($cyan)════════════════════════════════════════($reset)"
    print ""
    print "Files written:"
    print $"  ($green)Cargo.toml($reset)          version → ($version)"
    print $"  ($green)CHANGELOG.md($reset)        full history"
    print $"  ($green)RELEASE_NOTES.md($reset)    release body for ($tag)"
}
