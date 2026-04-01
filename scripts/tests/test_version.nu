#!/usr/bin/env nu
# Tests for scripts/version.nu
#
# Run with: nu scripts/tests/test_version.nu

use std/assert
use runner.nu *

# ── Helpers ───────────────────────────────────────────────────────────────────

# Extract the [package] version from a Cargo.toml string.
def parse_package_version [cargo_toml: string] {
    $cargo_toml
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

# Write a minimal single-crate Cargo.toml at the given version into a temp dir.
def make_package_cargo [version: string] {
    let tmp = (mktemp -d)
    let content = $'[package]
name = "tui-piechart"
version = "($version)"
edition = "2021"
license = "MIT"

[dependencies]
ratatui = { version = "0.30", default-features = false }
'
    $content | save --force ($tmp | path join "Cargo.toml")
    $tmp
}

# ── Version parsing tests ─────────────────────────────────────────────────────

def "test version reads simple semver" [] {
    let tmp = make_package_cargo "1.2.3"
    let got = parse_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert equal $got "1.2.3"
}

def "test version reads zero patch" [] {
    let tmp = make_package_cargo "0.1.0"
    let got = parse_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert equal $got "0.1.0"
}

def "test version reads triple zero" [] {
    let tmp = make_package_cargo "0.0.0"
    let got = parse_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert equal $got "0.0.0"
}

def "test version reads pre-release suffix" [] {
    let tmp = make_package_cargo "0.5.0-beta.1"
    let got = parse_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert equal $got "0.5.0-beta.1"
}

def "test version reads padded assignment" [] {
    let tmp = (mktemp -d)
    let content = '[package]
name = "tui-piechart"
version      = "2.0.0"
edition      = "2021"
'
    $content | save --force ($tmp | path join "Cargo.toml")
    let got = parse_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert equal $got "2.0.0"
}

def "test version ignores dependency version lines" [] {
    let tmp = (mktemp -d)
    let content = '[package]
name = "tui-piechart"
version = "3.1.4"
edition = "2021"

[dependencies]
ratatui    = { version = "0.30" }
color-eyre = { version = "0.6" }
'
    $content | save --force ($tmp | path join "Cargo.toml")
    let got = parse_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert equal $got "3.1.4"
}

def "test version does not bleed from other sections" [] {
    let tmp = (mktemp -d)
    let content = '[package]
name = "tui-piechart"
version = "0.3.1"
edition = "2021"

[some.other.section]
version = "99.0.0"
'
    $content | save --force ($tmp | path join "Cargo.toml")
    let got = parse_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert equal $got "0.3.1"
}

def "test version output contains no whitespace" [] {
    let tmp = make_package_cargo "0.9.1"
    let got = parse_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert ($got !~ '\s') $"expected no whitespace, got: ($got)"
}

def "test version output contains no quotes" [] {
    let tmp = make_package_cargo "1.0.0"
    let got = parse_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert ($got !~ '"') $"expected no quotes, got: ($got)"
}

def "test version returns empty for missing package section" [] {
    let tmp = (mktemp -d)
    let content = '[dependencies]
ratatui = "0.30"
'
    $content | save --force ($tmp | path join "Cargo.toml")
    let got = parse_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert ($got | is-empty) "should return empty string when section is absent"
}

def "test version native toml read matches text parse" [] {
    let tmp = make_package_cargo "1.7.3"
    let path = ($tmp | path join "Cargo.toml")
    let via_toml = (open $path | get package.version)
    let via_text = (parse_package_version (open --raw $path))
    rm -rf $tmp
    assert equal $via_toml $via_text
}

# ── Runner ────────────────────────────────────────────────────────────────────

def main [] {
    print $"(ansi cyan)═══ test_version.nu ═══(ansi reset)"
    run-tests
}
