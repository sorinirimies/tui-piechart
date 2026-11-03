#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Development tools: check or install
# ──────────────────────────────────────────────────────────────────────────────
# Usage:
#   nu scripts/tools.nu check git-cliff     fail with an install hint if missing
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
}

# rustup targets the build needs besides the host (e.g. wasm32-wasip2 for a Zed
# extension). `install` adds them; rustup skips the ones already there.
export const TARGETS = []

export def missing [installed: list<string>] {
    $TOOLS | columns | where {|t| $t not-in $installed }
}

def installed [] { $TOOLS | columns | where {|t| which $t | is-not-empty } }

def "main check" [tool: string] {
    if (which $tool | is-empty) {
        print $"❌ ($tool) not found. Install with: cargo install ($TOOLS | get -o $tool | default $tool) --locked"
        exit 1
    }
}

def "main install" [] {
    if ($TARGETS | is-not-empty) { ^rustup target add ...$TARGETS }
    for t in (missing (installed)) { ^cargo install ($TOOLS | get $t) --locked }
    print "✅ tools installed"
}

def main [] { print "usage: nu scripts/tools.nu (check <tool> | install)" }
