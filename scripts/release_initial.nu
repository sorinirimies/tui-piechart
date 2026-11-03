#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — First release: tag the version already in Cargo.toml (no bump)
# ──────────────────────────────────────────────────────────────────────────────
# Refuses a dirty tree or an existing tag (local or on origin); regenerates the
# changelog for the tag, commits it if it changed, tags. Pushing is the caller's.
#
# Usage: nu scripts/release_initial.nu
# ──────────────────────────────────────────────────────────────────────────────

use version.nu [read_version]

def fail [msg: string] { print $"(ansi red)❌ ($msg)(ansi reset)"; exit 1 }

def main [] {
    let tag = $"v(read_version)"
    ^nu scripts/ci/check_versions.nu $tag
    if (^git status --porcelain | str trim | is-not-empty) { fail "the working tree is dirty" }
    if (^git tag --list $tag | str trim | is-not-empty) { fail $"($tag) exists locally" }
    let remote = do { ^git ls-remote --tags origin $"refs/tags/($tag)" } | complete
    if $remote.exit_code == 0 and ($remote.stdout | str trim | is-not-empty) { fail $"($tag) exists on origin" }

    ^git-cliff --output CHANGELOG.md --tag $tag
    ^git add CHANGELOG.md
    if (do { ^git diff --cached --quiet } | complete | get exit_code) != 0 {
        ^git commit -q -m $"chore: release ($tag)"
    }
    ^git tag -a $tag -m $"Release ($tag)"
    print $"(ansi green)✓(ansi reset) tagged ($tag)"
}
