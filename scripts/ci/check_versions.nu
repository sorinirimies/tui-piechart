#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Versions agree
# ──────────────────────────────────────────────────────────────────────────────
# The workspace version (Cargo.toml), the top-level `version` of every file in
# EXTRA_MANIFESTS and, for a release, the tag must all be the same.
#
# Usage:
#   nu scripts/ci/check_versions.nu            # Cargo.toml == EXTRA_MANIFESTS
#   nu scripts/ci/check_versions.nu v0.2.0     # … and == the tag
# ──────────────────────────────────────────────────────────────────────────────

# Other manifests whose `version` must follow Cargo.toml (bump_version.nu edits
# them too), e.g. ["extension.toml"] for a Zed extension. Single source of truth.
export const EXTRA_MANIFESTS = []

use ../version.nu [cargo_version]

# The mismatches between the versions (empty = all agree). `manifests` maps a
# manifest path to its version.
export def mismatches [cargo: string, manifests: record, tag?: string]: nothing -> list<string> {
    mut out = []
    for e in ($manifests | transpose file version) {
        if $cargo != $e.version {
            $out = ($out | append $"Cargo.toml has ($cargo) but ($e.file) has ($e.version)")
        }
    }
    if $tag != null {
        let v = ($tag | str replace --regex '^v' '')
        if $v != $cargo {
            $out = ($out | append $"tag ($tag) does not match Cargo.toml ($cargo)")
        }
    }
    $out
}

def main [tag?: string] {
    let cargo = (cargo_version (open Cargo.toml))
    let manifests = ($EXTRA_MANIFESTS | reduce --fold {} {|f, acc| $acc | insert $f (open $f | get version) })
    let bad = (mismatches $cargo $manifests $tag)
    if ($bad | is-not-empty) {
        for b in $bad { print --stderr $"(ansi red)❌ ($b)(ansi reset)" }
        exit 1
    }
    print $"(ansi green)✅ versions agree: ($cargo)(ansi reset)"
}
