#!/usr/bin/env nu
# Tests for scripts/release_prepare.nu
#
# Run with: nu scripts/tests/test_release_prepare.nu

use std/assert
use runner.nu *

# ── Helpers ───────────────────────────────────────────────────────────────────

# Write a minimal single-crate Cargo.toml at the given version into a temp dir.
def make_package_cargo [version: string] {
    let tmp = (mktemp -d)
    let content = $'[package]
name = "tui-piechart"
version = "($version)"
authors = ["Test Author <test@example.com>"]
edition = "2021"
license = "MIT"
description = "A customizable pie chart widget for Ratatui"

[dependencies]
ratatui = { version = "0.30", default-features = false }
'
    $content | save --force ($tmp | path join "Cargo.toml")
    $tmp
}

# Read back the [package] version from a Cargo.toml file path.
def read_package_version [cargo_path: string] {
    open --raw $cargo_path
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

# Apply the same Cargo.toml update logic that release_prepare.nu uses.
def apply_version_update [dir: string, new_version: string] {
    let cargo_path = ($dir | path join "Cargo.toml")
    let lines = open --raw $cargo_path | lines

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

    $result.lines | str join "\n" | save --force $cargo_path
}

# Build the release notes string the same way release_prepare.nu does.
def build_release_notes [version: string, cliff_changes: string, last_tag: string] {
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

# ── Tag stripping tests ────────────────────────────────────────────────────────

def "test tag v prefix is stripped" [] {
    let tag = "v0.4.0"
    let version = $tag | str replace --regex '^v' ''
    assert equal $version "0.4.0"
}

def "test tag without v prefix is unchanged" [] {
    let tag = "1.2.3"
    let version = $tag | str replace --regex '^v' ''
    assert equal $version "1.2.3"
}

def "test tag with pre-release suffix strips v only" [] {
    let tag = "v0.4.0-beta.1"
    let version = $tag | str replace --regex '^v' ''
    assert equal $version "0.4.0-beta.1"
}

def "test tag name round trips from version" [] {
    let version = "2.0.0"
    let tag = $"v($version)"
    let back = $tag | str replace --regex '^v' ''
    assert equal $back $version
}

def "test tag with rc suffix is handled" [] {
    let tag = "v1.0.0-rc.1"
    let version = $tag | str replace --regex '^v' ''
    assert equal $version "1.0.0-rc.1"
}

# ── Cargo.toml update tests ───────────────────────────────────────────────────

def "test cargo package version is updated" [] {
    let tmp = make_package_cargo "0.3.1"
    apply_version_update $tmp "0.4.0"
    let got = read_package_version ($tmp | path join "Cargo.toml")
    rm -rf $tmp
    assert equal $got "0.4.0"
}

def "test cargo package version update is verified" [] {
    let tmp = make_package_cargo "0.3.1"
    apply_version_update $tmp "1.0.0"
    let got = read_package_version ($tmp | path join "Cargo.toml")
    rm -rf $tmp
    assert equal $got "1.0.0" "verification should pass when update succeeded"
}

def "test cargo non-version lines survive update" [] {
    let tmp = make_package_cargo "0.3.1"
    apply_version_update $tmp "0.4.0"
    let content = open --raw ($tmp | path join "Cargo.toml")
    rm -rf $tmp
    assert str contains $content "[package]"
    assert str contains $content "edition"
    assert str contains $content "ratatui"
}

def "test cargo dependency version lines are untouched" [] {
    let tmp = (mktemp -d)
    let content = '[package]
name = "tui-piechart"
version = "1.0.0"
edition = "2021"

[dependencies]
ratatui    = { version = "0.30" }
color-eyre = { version = "0.6" }
'
    $content | save --force ($tmp | path join "Cargo.toml")
    apply_version_update $tmp "1.1.0"
    let updated = open --raw ($tmp | path join "Cargo.toml")
    rm -rf $tmp
    assert str contains $updated 'version = "1.1.0"'
    assert str contains $updated 'version = "0.30"'
    assert str contains $updated 'version = "0.6"'
}

# ── Last-tag detection logic tests ────────────────────────────────────────────

def "test empty last tag triggers initial release header" [] {
    let last_tag = ""
    let header = if ($last_tag | is-empty) {
        "### 🎉 Initial Release"
    } else {
        $"### 📝 Changes since ($last_tag):"
    }
    assert equal $header "### 🎉 Initial Release"
}

def "test non-empty last tag triggers changes-since header" [] {
    let last_tag = "v0.3.1"
    let header = if ($last_tag | is-empty) {
        "### 🎉 Initial Release"
    } else {
        $"### 📝 Changes since ($last_tag):"
    }
    assert equal $header "### 📝 Changes since v0.3.1:"
}

def "test last tag is trimmed" [] {
    let raw = "v0.3.1\n"
    let trimmed = $raw | str trim
    assert equal $trimmed "v0.3.1"
}

def "test last tag trimmed empty is detected correctly" [] {
    let raw = "\n"
    let trimmed = $raw | str trim
    assert ($trimmed | is-empty) "trimmed empty string should be detected as empty"
}

# ── Release notes content tests ───────────────────────────────────────────────

def "test release notes contains version header" [] {
    let notes = build_release_notes "0.4.0" "- fix something" ""
    assert str contains $notes "# tui-piechart 0.4.0"
}

def "test release notes contains whats new section" [] {
    let notes = build_release_notes "0.4.0" "- fix something" ""
    assert str contains $notes "## 🚀 What's New"
}

def "test release notes initial release has correct header" [] {
    let notes = build_release_notes "0.1.0" "- initial" ""
    assert str contains $notes "### 🎉 Initial Release"
    assert not ($notes | str contains "### 📝 Changes since")
}

def "test release notes with previous tag has changes-since header" [] {
    let notes = build_release_notes "0.4.0" "- add feature" "v0.3.1"
    assert str contains $notes "### 📝 Changes since v0.3.1:"
    assert not ($notes | str contains "### 🎉 Initial Release")
}

def "test release notes contains cliff changes" [] {
    let cliff = "- feat: add cool feature\n- fix: patch a bug"
    let notes = build_release_notes "0.4.0" $cliff ""
    assert str contains $notes "feat: add cool feature"
    assert str contains $notes "fix: patch a bug"
}

def "test release notes contains installation section" [] {
    let notes = build_release_notes "0.4.0" "- changes" ""
    assert str contains $notes "## 📦 Installation"
}

def "test release notes contains cargo add command" [] {
    let notes = build_release_notes "0.4.0" "- changes" ""
    assert str contains $notes "cargo add tui-piechart"
}

def "test release notes contains cargo toml dep line" [] {
    let notes = build_release_notes "0.4.0" "- changes" ""
    assert str contains $notes "tui-piechart = \"0.4.0\""
}

def "test release notes contains quick start section" [] {
    let notes = build_release_notes "0.4.0" "- changes" ""
    assert str contains $notes "## 🚀 Quick Start"
}

def "test release notes contains piechart usage example" [] {
    let notes = build_release_notes "0.4.0" "- changes" ""
    assert str contains $notes "PieChart::new(slices)"
    assert str contains $notes "PieSlice::new"
}

def "test release notes is a single string" [] {
    let notes = build_release_notes "0.4.0" "- changes" ""
    assert ($notes | describe | str starts-with "string")
}

def "test release notes pre-release version is rendered correctly" [] {
    let notes = build_release_notes "1.0.0-rc.1" "- rc changes" "v0.9.9"
    assert str contains $notes "# tui-piechart 1.0.0-rc.1"
    assert str contains $notes "tui-piechart = \"1.0.0-rc.1\""
}

# ── RELEASE_NOTES.md file write tests ─────────────────────────────────────────

def "test release notes is written to file" [] {
    let tmp = (mktemp -d)
    let notes = build_release_notes "0.4.0" "- initial release" ""
    $notes | save --force ($tmp | path join "RELEASE_NOTES.md")
    assert (($tmp | path join "RELEASE_NOTES.md") | path exists)
    rm -rf $tmp
}

def "test release notes file content matches notes" [] {
    let tmp = (mktemp -d)
    let notes = build_release_notes "0.4.0" "- big release" "v0.3.1"
    $notes | save --force ($tmp | path join "RELEASE_NOTES.md")
    let content = open --raw ($tmp | path join "RELEASE_NOTES.md")
    rm -rf $tmp
    assert str contains $content "# tui-piechart 0.4.0"
    assert str contains $content "### 📝 Changes since v0.3.1:"
}

def "test changelog is written to file" [] {
    let tmp = (mktemp -d)
    let changelog = "# Changelog\n\n## v0.4.0\n\n- feat: new stuff"
    $changelog | save --force ($tmp | path join "CHANGELOG.md")
    assert (($tmp | path join "CHANGELOG.md") | path exists)
    let content = open --raw ($tmp | path join "CHANGELOG.md")
    rm -rf $tmp
    assert str contains $content "v0.4.0"
}

# ── Runner ────────────────────────────────────────────────────────────────────

def main [] {
    print $"(ansi cyan)═══ test_release_prepare.nu ═══(ansi reset)"
    run-tests
}
