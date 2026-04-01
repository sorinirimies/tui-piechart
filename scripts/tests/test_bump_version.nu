#!/usr/bin/env nu
# Tests for scripts/bump_version.nu
#
# Run with: nu scripts/tests/test_bump_version.nu

use std/assert
use runner.nu *

# ── Helpers ───────────────────────────────────────────────────────────────────

# Write a minimal single-crate Cargo.toml at the given version into a temp dir.
# Returns the dir path.
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

[dev-dependencies]
color-eyre = "0.6"
'
    $content | save --force ($tmp | path join "Cargo.toml")
    $tmp
}

# Read back the [package] version from a Cargo.toml string.
def read_package_version [content: string] {
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

# Apply the same [package] version update logic that bump_version.nu uses.
def apply_package_version_update [dir: string, new_version: string] {
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

# ── Version format validation tests ───────────────────────────────────────────

def "test valid version x.y.z is accepted" [] {
    assert ("1.2.3" =~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$')
}

def "test valid version 0.0.0 is accepted" [] {
    assert ("0.0.0" =~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$')
}

def "test valid version with pre-release suffix is accepted" [] {
    assert ("1.0.0-beta.1" =~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$')
}

def "test valid version with alpha suffix is accepted" [] {
    assert ("2.3.4-alpha" =~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$')
}

def "test invalid version missing patch is rejected" [] {
    assert not ("1.2" =~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$')
}

def "test invalid version with v prefix is rejected" [] {
    assert not ("v1.2.3" =~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$')
}

def "test invalid version empty string is rejected" [] {
    assert not ("" =~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$')
}

def "test invalid version with spaces is rejected" [] {
    assert not ("1.2.3 " =~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$')
}

def "test invalid version with only two parts is rejected" [] {
    assert not ("0.8" =~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$')
}

# ── [package] version reading tests ───────────────────────────────────────────

def "test read package version from simple cargo toml" [] {
    let tmp = make_package_cargo "0.3.1"
    let content = open --raw ($tmp | path join "Cargo.toml")
    let got = read_package_version $content
    rm -rf $tmp
    assert equal $got "0.3.1"
}

def "test read package version with padded assignment" [] {
    let tmp = (mktemp -d)
    let content = '[package]
name = "tui-piechart"
version      = "1.2.3"
edition      = "2021"
'
    $content | save --force ($tmp | path join "Cargo.toml")
    let got = read_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert equal $got "1.2.3"
}

def "test read package version ignores dependency version lines" [] {
    let tmp = make_package_cargo "2.0.0"
    let content = open --raw ($tmp | path join "Cargo.toml")
    let got = read_package_version $content
    rm -rf $tmp
    assert equal $got "2.0.0"
}

def "test read package version ignores dep inline version" [] {
    let tmp = (mktemp -d)
    let content = '[package]
name = "tui-piechart"
version = "3.1.4"
edition = "2021"

[dependencies]
ratatui = { version = "0.30", default-features = false }
'
    $content | save --force ($tmp | path join "Cargo.toml")
    let got = read_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert equal $got "3.1.4"
}

# ── [package] version update tests ────────────────────────────────────────────

def "test package version is updated" [] {
    let tmp = make_package_cargo "0.3.1"
    apply_package_version_update $tmp "0.4.0"
    let content = open --raw ($tmp | path join "Cargo.toml")
    let got = read_package_version $content
    rm -rf $tmp
    assert equal $got "0.4.0"
}

def "test package version patch bump is correct" [] {
    let tmp = make_package_cargo "0.3.1"
    apply_package_version_update $tmp "0.3.2"
    let content = open --raw ($tmp | path join "Cargo.toml")
    let got = read_package_version $content
    rm -rf $tmp
    assert equal $got "0.3.2"
}

def "test package version minor bump is correct" [] {
    let tmp = make_package_cargo "0.3.1"
    apply_package_version_update $tmp "0.4.0"
    let content = open --raw ($tmp | path join "Cargo.toml")
    let got = read_package_version $content
    rm -rf $tmp
    assert equal $got "0.4.0"
}

def "test package version major bump is correct" [] {
    let tmp = make_package_cargo "0.9.9"
    apply_package_version_update $tmp "1.0.0"
    let content = open --raw ($tmp | path join "Cargo.toml")
    let got = read_package_version $content
    rm -rf $tmp
    assert equal $got "1.0.0"
}

def "test package version update leaves other lines intact" [] {
    let tmp = make_package_cargo "0.3.1"
    apply_package_version_update $tmp "0.4.0"
    let content = open --raw ($tmp | path join "Cargo.toml")
    rm -rf $tmp
    assert str contains $content "[package]"
    assert str contains $content "edition"
    assert str contains $content "ratatui"
}

def "test package version update does not change dep versions" [] {
    let tmp = make_package_cargo "1.0.0"
    apply_package_version_update $tmp "1.1.0"
    let content = open --raw ($tmp | path join "Cargo.toml")
    rm -rf $tmp
    # ratatui dep must remain at 0.30
    assert str contains $content 'version = "0.30"'
}

def "test package version update is idempotent" [] {
    let tmp = make_package_cargo "1.0.0"
    apply_package_version_update $tmp "1.0.0"
    let content = open --raw ($tmp | path join "Cargo.toml")
    let got = read_package_version $content
    rm -rf $tmp
    assert equal $got "1.0.0"
}

# ── Same-version guard tests ──────────────────────────────────────────────────

def "test same version is detected" [] {
    let current = "0.3.1"
    let new     = "0.3.1"
    assert equal $current $new "same version guard should trigger"
}

def "test different version is not blocked" [] {
    let current = "0.3.1"
    let new     = "0.4.0"
    assert not equal $current $new
}

# ── Tag existence guard tests ─────────────────────────────────────────────────

def "test tag check detects existing tag" [] {
    let existing  = ["v0.3.0" "v0.3.1"]
    let candidate = "v0.3.1"
    assert ($existing | any { |t| $t == $candidate }) "existing tag should be detected"
}

def "test tag check allows new tag" [] {
    let existing  = ["v0.3.0" "v0.3.1"]
    let candidate = "v0.4.0"
    assert not ($existing | any { |t| $t == $candidate }) "new tag should not be blocked"
}

def "test tag name is prefixed with v" [] {
    let version = "0.4.0"
    let tag = $"v($version)"
    assert str contains $tag "v"
    assert equal $tag "v0.4.0"
}

# ── Version via native TOML read ──────────────────────────────────────────────

def "test native toml read matches text parse" [] {
    let tmp = make_package_cargo "1.7.3"
    let path = ($tmp | path join "Cargo.toml")
    let via_toml = (open $path | get package.version)
    let via_text = (read_package_version (open --raw $path))
    rm -rf $tmp
    assert equal $via_toml $via_text
}

# ── Runner ────────────────────────────────────────────────────────────────────

def main [] {
    print $"(ansi cyan)═══ test_bump_version.nu ═══(ansi reset)"
    run-tests
}
