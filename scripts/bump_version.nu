#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Bump the version
# ──────────────────────────────────────────────────────────────────────────────
# Usage:
#   nu scripts/bump_version.nu <new_version> [--yes] [--skip-checks] [--check]
#
# One version, everywhere (the release workflow checks it):
#   * the version in Cargo.toml: `[workspace.package]` (every crate inherits it),
#     or `[package]` for a single crate at the root
#   * the top-level `version` of each file in check_versions.nu's EXTRA_MANIFESTS
#     (e.g. a Zed extension.toml, a package.json)
#   * Cargo.lock
#
# Order: validate (cheap) → quality gate on the untouched tree → edit, changelog,
# commit and tag. Any failure after the first edit is rolled back, so a failed
# bump never leaves a half-bumped tree behind.
#
# Flags:
#   --check        only validate (well-formed, strictly newer), then exit
#   --yes          non-interactive
#   --skip-checks  the caller already ran the gate (`just bump` runs check-all)
# ──────────────────────────────────────────────────────────────────────────────

# Is `version` MAJOR.MINOR.PATCH with an optional pre-release?
export def is_semver [version: string]: nothing -> bool {
    $version =~ '^\d+\.\d+\.\d+(-[a-zA-Z0-9.]+)?$'
}

# Compare two semvers: 1 if a > b, -1 if a < b, 0 if equal (pre-releases sort
# below their release, identifiers compare numerically when both are numbers).
export def compare_versions [a: string, b: string]: nothing -> int {
    let core = {|v| $v | split row "-" | first | split row "." | each {|x| $x | into int } }
    let pa = (do $core $a)
    let pb = (do $core $b)
    for i in 0..2 {
        if ($pa | get $i) > ($pb | get $i) { return 1 }
        if ($pa | get $i) < ($pb | get $i) { return (-1) }
    }
    let ra = ($a | split row "-" | skip 1 | str join "-")
    let rb = ($b | split row "-" | skip 1 | str join "-")
    if $ra == $rb { return 0 }
    if ($ra | is-empty) { return 1 }
    if ($rb | is-empty) { return (-1) }
    let ia = ($ra | split row ".")
    let ib = ($rb | split row ".")
    let n = ([($ia | length) ($ib | length)] | math min)
    for i in 0..<$n {
        let x = ($ia | get $i)
        let y = ($ib | get $i)
        if $x == $y { continue }
        let xn = ($x =~ '^\d+$')
        let yn = ($y =~ '^\d+$')
        if $xn and $yn { return (if ($x | into int) > ($y | into int) { 1 } else { -1 }) }
        if $xn { return (-1) }
        if $yn { return 1 }
        return (if $x > $y { 1 } else { -1 })
    }
    if ($ia | length) > ($ib | length) { 1 } else if ($ia | length) < ($ib | length) { -1 } else { 0 }
}

# Set `version` inside `[workspace.package]` of a Cargo.toml text, or inside
# `[package]` when there is no workspace version (only there: dependency
# versions are left alone).
export def set_workspace_version [text: string, version: string]: nothing -> string {
    let section = if ($text | lines | any {|l| ($l | str trim) == "[workspace.package]" }) { "[workspace.package]" } else { "[package]" }
    mut in_section = false
    mut out = []
    for line in ($text | lines) {
        let trimmed = ($line | str trim)
        if ($trimmed | str starts-with "[") {
            $in_section = ($trimmed == $section)
        }
        if $in_section and ($trimmed =~ '^version\s*=') {
            $out = ($out | append ($line | str replace --regex '"[^"]*"' $'"($version)"'))
        } else {
            $out = ($out | append $line)
        }
    }
    ($out | str join "\n") + "\n"
}

# Set the first top-level `version = "…"` of a TOML text (extension.toml & co).
export def set_extension_version [text: string, version: string]: nothing -> string {
    mut done = false
    mut out = []
    for line in ($text | lines) {
        if (not $done) and ($line =~ '^version\s*=') {
            $out = ($out | append $'version = "($version)"')
            $done = true
        } else {
            $out = ($out | append $line)
        }
    }
    ($out | str join "\n") + "\n"
}

use ci/check_versions.nu [EXTRA_MANIFESTS]
use version.nu [cargo_version]

const TOUCHED = ["Cargo.toml" "Cargo.lock" "fuzz/Cargo.lock" "CHANGELOG.md"]

def run_checks [] {
    print $"(ansi cyan)── quality gate: nothing is bumped until it is green ──(ansi reset)"
    nu scripts/ci/quality_gate.nu
}

def roll_back [start: string, tag: string] {
    print $"(ansi red)✗ Bump failed — rolling back.(ansi reset)"
    do --ignore-errors { ^git tag -d $tag | ignore }
    ^git reset --hard $start
}

def main [
    new_version: string,  # New version, X.Y.Z
    --yes (-y),           # Non-interactive
    --skip-checks,        # The caller already ran the quality gate
    --check,              # Only validate, then exit
] {
    let current = (cargo_version (open Cargo.toml))
    let tag = $"v($new_version)"
    print $"  Current version : (ansi yellow)($current)(ansi reset)"
    print $"  New version     : (ansi green)($new_version)(ansi reset)"

    if not (is_semver $new_version) {
        print $"(ansi red)Error:(ansi reset) '($new_version)' is not a valid semantic version."
        exit 1
    }
    let cmp = (compare_versions $new_version $current)
    if $cmp == 0 {
        print $"(ansi red)Error:(ansi reset) ($new_version) is already the current version."
        exit 1
    }
    if $cmp < 0 {
        print $"(ansi red)Error:(ansi reset) ($new_version) is older than ($current); a bump must go up."
        exit 1
    }
    print $"(ansi green)✓(ansi reset) ($new_version) is valid and newer than ($current)."
    if $check { return }

    if (^git tag --list $tag | str trim | is-not-empty) {
        print $"(ansi red)Error:(ansi reset) tag ($tag) already exists."
        exit 1
    }
    # A tag someone else pushed would make our push fail after the commit.
    let remote = do { ^git ls-remote --tags origin $"refs/tags/($tag)" } | complete
    if $remote.exit_code == 0 and ($remote.stdout | str trim | is-not-empty) {
        print $"(ansi red)Error:(ansi reset) tag ($tag) already exists on origin."
        exit 1
    }
    if (^git status --porcelain | str trim | is-not-empty) {
        print $"(ansi red)Error:(ansi reset) the working tree is dirty — commit or stash first."
        exit 1
    }
    if not $yes {
        let answer = (input $"Bump to ($new_version), commit and tag ($tag)? [y/N] ")
        if ($answer | str lowercase) != "y" { print "Aborted."; exit 1 }
    }
    let start = (^git rev-parse HEAD | str trim)

    if $skip_checks {
        print $"(ansi yellow)⚠(ansi reset) --skip-checks: assuming the gate already ran."
    } else {
        run_checks
    }

    try {
        set_workspace_version (open Cargo.toml --raw) $new_version | save --force Cargo.toml
        for f in $EXTRA_MANIFESTS {
            set_extension_version (open $f --raw) $new_version | save --force $f
        }
        print $"(ansi green)✓(ansi reset) Cargo.toml ($EXTRA_MANIFESTS | str join ' ') → ($new_version)"

        ^cargo update --workspace
        ^cargo metadata --locked --format-version 1 | ignore
        # fuzz/ is its own workspace and records our crate's version too; resolving
        # it refreshes the path dependency without upgrading anything else.
        if ("fuzz/Cargo.toml" | path exists) {
            do { cd fuzz; ^cargo metadata --format-version 1 | ignore }
        }
        print $"(ansi green)✓(ansi reset) Cargo.lock updated."

        if (which git-cliff | is-not-empty) {
            ^git-cliff --output CHANGELOG.md --tag $tag
            print $"(ansi green)✓(ansi reset) CHANGELOG.md regenerated."
        } else {
            print $"(ansi yellow)⚠(ansi reset) git-cliff not found — CHANGELOG.md unchanged."
        }

        ^git add ...($TOUCHED | where {|f| $f | path exists }) ...$EXTRA_MANIFESTS
        ^git commit -m $"chore: bump version to ($new_version)"
        ^git tag -a $tag -m $"Release ($tag)"
        print $"(ansi green)✓(ansi reset) Committed and tagged ($tag)."
    } catch {|err|
        roll_back $start $tag
        error make --unspanned { msg: $"bump to ($new_version) failed and was rolled back: ($err.msg)" }
    }
    print $"  Next: git push --follow-tags origin main"
}
