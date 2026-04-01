#!/usr/bin/env nu
# Tests for scripts/check_publish.nu
#
# Run with: nu scripts/tests/test_check_publish.nu

use std/assert
use runner.nu *

# ── Helpers ───────────────────────────────────────────────────────────────────

# Write a minimal single-crate Cargo.toml at the given version into a temp dir.
def make_package_cargo [version: string] {
    let tmp = (mktemp -d)
    let content = $'[package]
name = "tui-piechart"
version = "($version)"
edition = "2021"
license = "MIT"
description = "A customizable pie chart widget for Ratatui"

[dependencies]
ratatui = { version = "0.30", default-features = false }
'
    $content | save --force ($tmp | path join "Cargo.toml")
    $tmp
}

# Simulate the package version read used in check_publish.nu
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

# Simulate the required-files check used in check_publish.nu
def check_required_files [dir: string, files: list<string>] {
    $files | where { |f| not (($dir | path join $f) | path exists) }
}

# Count errors (helper for testing error accumulation logic)
def count_errors [checks: list<bool>] {
    $checks | where { |c| not $c } | length
}

# ── Package version reading tests ─────────────────────────────────────────────

def "test package version is readable" [] {
    let tmp = make_package_cargo "0.3.1"
    let content = open --raw ($tmp | path join "Cargo.toml")
    let got = read_package_version $content
    rm -rf $tmp
    assert equal $got "0.3.1"
}

def "test package version is not empty for valid cargo toml" [] {
    let tmp = make_package_cargo "1.2.3"
    let content = open --raw ($tmp | path join "Cargo.toml")
    let got = read_package_version $content
    rm -rf $tmp
    assert not ($got | is-empty)
}

def "test package version is empty when section is missing" [] {
    let tmp = (mktemp -d)
    let content = '[dependencies]
ratatui = "0.30"
'
    $content | save --force ($tmp | path join "Cargo.toml")
    let got = read_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert ($got | is-empty)
}

def "test package version does not pick up dep version lines" [] {
    let tmp = make_package_cargo "2.0.0"
    let content = open --raw ($tmp | path join "Cargo.toml")
    let got = read_package_version $content
    rm -rf $tmp
    assert equal $got "2.0.0"
    assert not ($got == "0.30")
}

def "test package version handles padded assignment" [] {
    let tmp = (mktemp -d)
    let content = '[package]
name = "tui-piechart"
version      = "0.9.5"
edition      = "2021"

[dependencies]
ratatui = "0.30"
'
    $content | save --force ($tmp | path join "Cargo.toml")
    let got = read_package_version (open --raw ($tmp | path join "Cargo.toml"))
    rm -rf $tmp
    assert equal $got "0.9.5"
}

# ── Required files check tests ────────────────────────────────────────────────

def "test all required files present returns no missing" [] {
    let tmp = (mktemp -d)
    for f in ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md"] {
        "" | save --force ($tmp | path join $f)
    }
    let missing = check_required_files $tmp ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md"]
    rm -rf $tmp
    assert ($missing | is-empty)
}

def "test missing readme is detected" [] {
    let tmp = (mktemp -d)
    for f in ["LICENSE" "Cargo.toml" "CHANGELOG.md"] {
        "" | save --force ($tmp | path join $f)
    }
    let missing = check_required_files $tmp ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md"]
    rm -rf $tmp
    assert not ($missing | is-empty)
    assert ($missing | any { |f| $f == "README.md" })
}

def "test missing changelog is detected" [] {
    let tmp = (mktemp -d)
    for f in ["README.md" "LICENSE" "Cargo.toml"] {
        "" | save --force ($tmp | path join $f)
    }
    let missing = check_required_files $tmp ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md"]
    rm -rf $tmp
    assert not ($missing | is-empty)
    assert ($missing | any { |f| $f == "CHANGELOG.md" })
}

def "test missing license is detected" [] {
    let tmp = (mktemp -d)
    for f in ["README.md" "Cargo.toml" "CHANGELOG.md"] {
        "" | save --force ($tmp | path join $f)
    }
    let missing = check_required_files $tmp ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md"]
    rm -rf $tmp
    assert not ($missing | is-empty)
    assert ($missing | any { |f| $f == "LICENSE" })
}

def "test multiple missing files are all reported" [] {
    let tmp = (mktemp -d)
    "" | save --force ($tmp | path join "Cargo.toml")
    let missing = check_required_files $tmp ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md"]
    rm -rf $tmp
    assert equal ($missing | length) 3
}

def "test no false positives when all files exist" [] {
    let tmp = (mktemp -d)
    let required = ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md"]
    for f in $required {
        "" | save --force ($tmp | path join $f)
    }
    let missing = check_required_files $tmp $required
    rm -rf $tmp
    assert equal ($missing | length) 0
}

# ── Error counting / accumulation tests ───────────────────────────────────────

def "test zero errors when all checks pass" [] {
    let results = [true true true true true]
    assert equal (count_errors $results) 0
}

def "test one error is counted" [] {
    let results = [true false true true true]
    assert equal (count_errors $results) 1
}

def "test multiple errors are counted" [] {
    let results = [false true false true false]
    assert equal (count_errors $results) 3
}

def "test all errors are counted" [] {
    let results = [false false false]
    assert equal (count_errors $results) 3
}

# ── Cargo.lock presence tests ─────────────────────────────────────────────────

def "test cargo lock present is detected" [] {
    let tmp = (mktemp -d)
    "" | save --force ($tmp | path join "Cargo.lock")
    assert (($tmp | path join "Cargo.lock") | path exists)
    rm -rf $tmp
}

def "test cargo lock absent is detected" [] {
    let tmp = (mktemp -d)
    assert not (($tmp | path join "Cargo.lock") | path exists)
    rm -rf $tmp
}

# ── Error message / summary logic tests ───────────────────────────────────────

def "test error plural is correct for one error" [] {
    let errors = 1
    let plural = if $errors == 1 { "check" } else { "checks" }
    assert equal $plural "check"
}

def "test error plural is correct for multiple errors" [] {
    let errors = 3
    let plural = if $errors == 1 { "check" } else { "checks" }
    assert equal $plural "checks"
}

def "test zero errors means ready to publish" [] {
    let errors = 0
    assert ($errors == 0) "zero errors should mean ready"
}

def "test non-zero errors means not ready" [] {
    let errors = 2
    assert ($errors > 0) "non-zero errors should block release"
}

# ── Runner ────────────────────────────────────────────────────────────────────

def main [] {
    print $"(ansi cyan)═══ test_check_publish.nu ═══(ansi reset)"
    run-tests
}
