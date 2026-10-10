#!/usr/bin/env nu
# Nightly dependency upgrade for tui-piechart.
#
# Phases:
#   1. cargo upgrade (compatible-only)
#      Rewrites version pins in [dependencies] / [dev-dependencies] within
#      their existing semver range. Major bumps are a deliberate manual step.
#   2. cargo update
#      Resolves the (possibly new) constraints into a fresh Cargo.lock.
#      Runs unconditionally — also picks up compatible patch bumps when
#      Phase 1 produced no Cargo.toml changes.
#   3. Quality gate: fmt → clippy → tests
#   4. Commit strategy:
#      • Gate passed  + dep requirements changed → commit Cargo.toml + Cargo.lock,
#                                            bump PATCH version, tag, push and
#                                            emit `release_tag` (GITHUB_OUTPUT)
#      • Gate failed  + Cargo.toml changed → revert Cargo.toml, re-sync lock,
#                                            commit lock only, then exit 1
#      • Gate passed  + only Cargo.lock   → commit Cargo.lock only. NO release.
#      • Cosmetic Cargo.toml reformatting by cargo-edit is discarded.
#      • Nothing changed                  → nothing to commit, exit 0
#
# Usage:
#   nu scripts/upgrade_deps.nu
#   nu scripts/upgrade_deps.nu --bot-name "github-actions[bot]" \
#                               --bot-email "github-actions[bot]@users.noreply.github.com"
#   nu scripts/upgrade_deps.nu --dry-run      # show plan, do not commit or push

# ── Helpers ───────────────────────────────────────────────────────────────────

# Run a command captured in a closure, print a labelled pass/fail line.
# Returns true on success, false on failure.  Never throws.
def run_check [
    label:  string   # Display label printed before the result
    action: closure  # Command to run
] {
    let green = (ansi green)
    let red   = (ansi red)
    let reset = (ansi reset)

    print -n $"  ($label) ... "
    let result = (do $action | complete)

    if $result.exit_code == 0 {
        print $"($green)✓($reset)"
        true
    } else {
        print $"($red)✗($reset)"
        # Surface up to 20 lines of stderr/stdout so the failure is visible
        let out = if not ($result.stderr | is-empty) { $result.stderr } else { $result.stdout }
        $out | lines | first 20 | each { |l| print $"    ($l)" }
        false
    }
}

# True when the given path has uncommitted modifications in the working tree.
def is_dirty [path: string] {
    let r = (do { run-external "git" "diff" "--quiet" $path } | complete)
    $r.exit_code != 0
}

# ── Downgrade guard ───────────────────────────────────────────────────────────
# `cargo upgrade`/`cargo update` should only ever move dependency versions
# forward. If the local registry index/cache is stale they can silently pick
# an older version than what's already pinned (seen in practice: ratatui
# 0.30.2 -> 0.29.0, plus 8 transitive deps, with fmt/clippy/tests all still
# passing on the older versions). These pure helpers detect that case so the
# caller can abort before committing/pushing a downgrade.

# Parse a Cargo.lock file into a record mapping package name -> version.
# When a package name appears more than once (multiple semver-incompatible
# versions resolved simultaneously) the last occurrence wins, matching the
# order Cargo itself writes the lockfile in.
export def parse_lockfile [path: string] {
    open $path --raw
    | lines
    | reduce --fold {name: null, map: {}} {|line, acc|
        let l = ($line | str trim)
        if ($l | str starts-with 'name = "') {
            let n = ($l | parse --regex '"(?P<n>[^"]+)"' | get n | first)
            {name: $n, map: $acc.map}
        } else if ($l | str starts-with 'version = "') and ($acc.name != null) {
            let v = ($l | parse --regex '"(?P<v>[^"]+)"' | get v | first)
            {name: null, map: ($acc.map | upsert $acc.name $v)}
        } else {
            $acc
        }
    }
    | get map
}

# Turn a semver-ish version string into a list<int> for ordinal comparison,
# e.g. "0.30.2" -> [0, 30, 2]. Pre-release/build metadata (after the first
# "-" or "+") is dropped; non-numeric segments compare as 0.
export def version_key [v: string] {
    $v
    | split row "-" | get 0
    | split row "+" | get 0
    | split row "."
    | each {|p| if ($p =~ '^[0-9]+$') { $p | into int } else { 0 } }
}

# True when list `a` sorts strictly before list `b`, lexicographically,
# padding whichever list is shorter with trailing zeros.
export def list_less_than [a: list, b: list] {
    let max_len = ([($a | length) ($b | length)] | math max)
    let pairs = (0..<$max_len | each {|i|
        let av = if $i < ($a | length) { $a | get $i } else { 0 }
        let bv = if $i < ($b | length) { $b | get $i } else { 0 }
        {av: $av, bv: $bv}
    })
    let first_diff = ($pairs | where {|r| $r.av != $r.bv} | first)
    if ($first_diff | is-empty) {
        false
    } else {
        $first_diff.av < $first_diff.bv
    }
}

# Compare two { name -> version } records and return a table of every
# package whose version decreased from `old` to `new`. Packages only present
# in one side (added/removed deps) are not downgrades and are ignored.
export def find_downgrades [old: record, new: record] {
    $old
    | transpose name old_version
    | each {|row|
        let nv = ($new | get -o $row.name)
        if ($nv != null) and (list_less_than (version_key $nv) (version_key $row.old_version)) {
            {name: $row.name, old: $row.old_version, new: $nv}
        } else {
            null
        }
    }
    | compact
}

# Return the short commit label string for the given dirty state.
# Returns an empty string when nothing needs committing.
export def commit_label [toml_dirty: bool, lock_dirty: bool, date: string] {
    if $toml_dirty {
        $"chore: nightly dependency upgrade ($date)"
    } else if $lock_dirty {
        $"chore: nightly dependency update ($date)"
    } else {
        ""
    }
}

# Snapshot of the *semantic* dependency requirements declared in Cargo.toml
# (name, req, kind, features, ...). Unlike a text diff this ignores whitespace
# / key-order rewrites by cargo-edit, so it only differs on real changes.
export def deps_snapshot [] {
    run-external "cargo" "metadata" "--no-deps" "--format-version" "1"
    | from json
    | get packages
    | each {|p| $p.dependencies | select name req kind features optional target uses_default_features }
    | flatten
    | sort-by name kind
    | to json --raw
}

# Next PATCH version for a plain X.Y.Z string, e.g. "0.4.2" -> "0.4.3".
export def next_patch_version [v: string] {
    let p = ($v | split row "-" | get 0 | split row ".")
    let patch = (($p | get 2 | into int) + 1)
    $"($p | get 0).($p | get 1).($patch)"
}

# Return true when every element of a bool list is true.
export def all_passed [results: list<bool>] {
    $results | all { |x| $x }
}

# ── Main ──────────────────────────────────────────────────────────────────────

def main [
    --bot-name: string  = "github-actions[bot]"                           # Git commit author name
    --bot-email: string = "github-actions[bot]@users.noreply.github.com"  # Git commit author email
    --remote: string    = "origin"                                         # Git remote to push to
    --dry-run (-n)                                                         # Show what would be committed without pushing
] {
    let green  = (ansi green)
    let red    = (ansi red)
    let yellow = (ansi yellow)
    let cyan   = (ansi cyan)
    let reset  = (ansi reset)

    print $"($cyan)════════════════════════════════════════($reset)"
    print $"($cyan)  tui-piechart — Nightly Dependency Upgrade($reset)"
    print $"($cyan)════════════════════════════════════════($reset)"
    print ""

    # ── Phase 1: Upgrade Cargo.toml pins ─────────────────────────────────────
    # cargo-edit's `cargo upgrade` rewrites the version strings in Cargo.toml
    # ([dependencies] and [dev-dependencies]) within their semver range.
    # The quality gate below acts as the safety net.
    print $"($cyan)Phase 1/2 — Upgrading Cargo.toml dependency pins...($reset)"

    let lockfile_before = (parse_lockfile "Cargo.lock")
    let deps_before     = (deps_snapshot)

    let upgrade_result = (do {
        run-external "cargo" "upgrade"
    } | complete)

    let upgrade_log = ($upgrade_result.stdout | str trim)

    if $upgrade_result.exit_code != 0 {
        print $"($yellow)⚠ cargo upgrade exited non-zero:($reset)"
        $upgrade_result.stderr | lines | first 10 | each { |l| print $"  ($l)" }
    }

    let toml_changed = (is_dirty "Cargo.toml")

    if $toml_changed {
        print $"($green)✓ Cargo.toml pins updated($reset)"
        print ""
        # Echo what changed for visibility in CI logs
        do { run-external "git" "diff" "Cargo.toml" } | complete | null
    } else {
        print "  No Cargo.toml pin changes."
    }

    # ── Phase 2: Sync Cargo.lock ──────────────────────────────────────────────
    print ""
    print $"($cyan)Phase 2/2 — Syncing Cargo.lock...($reset)"
    run-external "cargo" "update"
    print $"($green)✓ Cargo.lock synced($reset)"

    # ── Guard against accidental downgrades ──────────────────────────────────
    print ""
    print $"($cyan)Downgrade guard:($reset)"

    let lockfile_after = (parse_lockfile "Cargo.lock")
    let downgrades = (find_downgrades $lockfile_before $lockfile_after)

    if not ($downgrades | is-empty) {
        print $"($red)✗ Detected dependency downgrade\(s\) — aborting without committing:($reset)"
        for d in $downgrades {
            print $"  ($d.name): ($d.old) -> ($d.new)"
        }
        print ""
        print $"($yellow)Restoring Cargo.toml and Cargo.lock to their pre-upgrade state.($reset)"
        run-external "git" "checkout" "--" "Cargo.toml" "Cargo.lock"
        exit 1
    }

    print $"($green)✓ No downgrades detected($reset)"

    # ── Quality gate ──────────────────────────────────────────────────────────
    print ""
    print $"($cyan)Quality gate:($reset)"

    let fmt    = (run_check "fmt    " { run-external "cargo" "fmt" "--" "--check" })
    let clippy = (run_check "clippy " { run-external "cargo" "clippy" "--" "-D" "warnings" })
    let tests  = (run_check "tests  " { run-external "cargo" "test" "--locked" "--all-features" "--all-targets" })

    let gate_passed = (all_passed [$fmt $clippy $tests])

    print ""
    if $gate_passed {
        print $"($green)✓ Quality gate passed($reset)"
    } else {
        print $"($red)✗ Quality gate failed($reset)"
    }

    # ── Revert Cargo.toml on quality-gate failure ─────────────────────────────
    # If the pin upgrade broke anything, roll back Cargo.toml to HEAD and
    # re-sync Cargo.lock so we can still commit a compatible patch-level update.
    # The job is marked as failed at the end so the developer is notified.
    if $toml_changed and (not $gate_passed) {
        print ""
        print $"($yellow)⚠ Reverting Cargo.toml — upgrade broke the quality gate.($reset)"
        print $"($yellow)  A compatible Cargo.lock patch update will still be committed.($reset)"
        run-external "git" "checkout" "Cargo.toml"
        run-external "cargo" "update"
        print $"($green)✓ Cargo.toml reverted and Cargo.lock re-synced($reset)"
    }

    # ── Commit strategy ───────────────────────────────────────────────────────
    # A release is cut ONLY when the dependency requirements declared in
    # Cargo.toml really changed (consumers resolve from those, never from our
    # Cargo.lock). Lock-only refreshes are committed but never released.
    let reqs_changed = ((deps_snapshot) != $deps_before)

    # Discard cosmetic-only Cargo.toml rewrites (reformat with no real change).
    if (is_dirty "Cargo.toml") and (not $reqs_changed) and (not $dry_run) {
        run-external "git" "checkout" "--" "Cargo.toml"
    }

    let toml_dirty = (is_dirty "Cargo.toml")
    let lock_dirty = (is_dirty "Cargo.lock")
    let date       = (date now | format date "%Y-%m-%d")
    let label      = (commit_label $toml_dirty $lock_dirty $date)

    print ""
    print $"($cyan)Commit strategy:($reset)"

    mut release_tag = ""

    if ($label | is-empty) {
        print $"  ($green)✓ Everything already up to date — nothing to commit($reset)"

    } else if $dry_run {
        let files = if $toml_dirty { "Cargo.toml + Cargo.lock" } else { "Cargo.lock" }
        let rel   = if $reqs_changed { "yes (patch release)" } else { "no" }
        print $"  ($cyan)[dry-run] Would commit: ($label)($reset)"
        print $"  ($cyan)[dry-run] Files: ($files)($reset)"
        print $"  ($cyan)[dry-run] Release: ($rel)($reset)"

    } else {
        run-external "git" "config" "user.name"  $bot_name
        run-external "git" "config" "user.email" $bot_email

        if $toml_dirty {
            run-external "git" "add" "Cargo.toml" "Cargo.lock" "src/" "examples/"
            # Include a summary of what was upgraded in the commit body (cap at 60 lines)
            let body = ($upgrade_log | lines | first 60 | str join "\n")
            if ($body | is-empty) {
                run-external "git" "commit" "-m" $label
            } else {
                run-external "git" "commit" "-m" $label "-m" $body
            }
        } else {
            run-external "git" "add" "Cargo.lock" "src/" "examples/"
            run-external "git" "commit" "-m" $label
        }

        if $reqs_changed and $gate_passed {
            let current = (open Cargo.toml --raw | lines
                | where { |l| $l =~ '^version\s*=' } | first
                | parse --regex '"(?P<v>[^"]+)"' | get v | first)
            let next = (next_patch_version $current)
            print $"  ($cyan)Dependency requirements changed → patch release ($current) → ($next)($reset)"
            run-external "nu" "scripts/bump_version.nu" "--yes" $next
            run-external "git" "push" "--follow-tags" $remote "main"
            $release_tag = $"v($next)"
            print $"($green)✓ Committed, tagged and pushed: ($label) + ($release_tag)($reset)"
        } else {
            run-external "git" "push" $remote "main"
            print $"($green)✓ Committed and pushed \(no release\): ($label)($reset)"
        }
    }

    # Expose the tag to the calling workflow (empty = no release).
    if ($env.GITHUB_OUTPUT? | is-not-empty) {
        $"release_tag=($release_tag)\n" | save --append $env.GITHUB_OUTPUT
    }

    # ── Summary ───────────────────────────────────────────────────────────────
    print ""
    print $"($cyan)════════════════════════════════════════($reset)"

    if $toml_changed and (not $gate_passed) {
        print $"($red)✗ Pin upgrades introduced breaking changes — Cargo.toml reverted.($reset)"
        print $"($yellow)  Fix the breaking changes and re-run, or pin the affected crate:($reset)"
        print "    • Update the source to work with the new API, or"
        print "    • Lower the version pin in [dependencies] in Cargo.toml."
        exit 1
    }

    print $"($green)✓ Nightly update complete!($reset)"
    print ""
}
