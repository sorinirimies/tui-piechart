#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Development tools: check or install
# ──────────────────────────────────────────────────────────────────────────────
# Usage:
#   nu scripts/tools.nu check git-cliff     fail with an install hint if missing
#   nu scripts/tools.nu ensure cargo-machete install it only if it is missing (the gate's tools)
#   nu scripts/tools.nu install             add the rustup TARGETS, cargo install every missing tool
# ──────────────────────────────────────────────────────────────────────────────

# command → crate to `cargo install`
export const TOOLS = {
    git-cliff: "git-cliff"
    cargo-audit: "cargo-audit"
    cargo-deny: "cargo-deny"
    cargo-fuzz: "cargo-fuzz"
    cargo-llvm-cov: "cargo-llvm-cov"
    cargo-outdated: "cargo-outdated"
    cargo-machete: "cargo-machete"
}

# rustup targets the build needs besides the host (e.g. wasm32-wasip2 for a Zed
# extension). `install` adds them; rustup skips the ones already there.
export const TARGETS = []

export def missing [installed: list<string>] {
    $TOOLS | columns | where {|t| $t not-in $installed }
}

# `cargo` arguments that install a tool (the crate from TOOLS, else the tool name).
export def install_args [tool: string]: nothing -> list<string> {
    [install ($TOOLS | get -o $tool | default $tool) --locked]
}

def installed [] { $TOOLS | columns | where {|t| which $t | is-not-empty } }

def "main check" [tool: string] {
    if (which $tool | is-empty) {
        print $"❌ ($tool) not found. Install with: cargo (install_args $tool | str join ' ')"
        exit 1
    }
}

def "main install" [] {
    if ($TARGETS | is-not-empty) { ^rustup target add ...$TARGETS }
    for t in (missing (installed)) { ^cargo ...(install_args $t) }
    print "✅ tools installed"
}

# The gate runs in CI, the nightlies and before a bump: install what it needs there.
def "main ensure" [tool: string] {
    if (which $tool | is-empty) { ^cargo ...(install_args $tool) }
}

def main [] { print "usage: nu scripts/tools.nu (check <tool> | ensure <tool> | install)" }
