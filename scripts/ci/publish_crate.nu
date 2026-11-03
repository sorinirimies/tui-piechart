#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Publish a crate to crates.io, idempotently
# ──────────────────────────────────────────────────────────────────────────────
# No CARGO_REGISTRY_TOKEN ⇒ skip (forks, first setup). "already exists" ⇒ success,
# so a re-run release (just release-retrigger) doesn't fail at the last step.
# Multi-crate workspaces: call it per crate, dependencies first.
#
# Usage: nu scripts/ci/publish_crate.nu <crate>
# ──────────────────────────────────────────────────────────────────────────────

export def already_published [output: string] {
    $output =~ '(already exists|already uploaded)'
}

def main [crate: string] {
    if ($env.CARGO_REGISTRY_TOKEN? | default "" | is-empty) {
        print "CARGO_REGISTRY_TOKEN not set — skipping crates.io"
        return
    }
    let r = do { ^cargo publish -p $crate --locked } | complete
    print $r.stdout $r.stderr
    if $r.exit_code == 0 { return }
    if (already_published $"($r.stdout)($r.stderr)") {
        print $"($crate) is already on crates.io — nothing to do"
        return
    }
    exit 1
}
