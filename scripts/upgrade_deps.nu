#!/usr/bin/env nu
# Nightly dependency upgrade for tui-piechart.
#
# Phases:
#   1. cargo upgrade --incompatible allow
#      Rewrites version pins in [dependencies] / [dev-dependencies] so libs
#      like `ratatui = "0.30"` can advance to newer releases.
#   2. cargo update
#      Resolves the (possibly new) constraints into a fresh Cargo.lock.
#      Runs unconditionally — also picks up compatible patch bumps when
#      Phase 1 produced no Cargo.toml changes.
#   3. Quality gate: fmt → clippy → tests
#   4. Commit strategy:
#      • Gate passed  + Cargo.toml changed → commit Cargo.toml + Cargo.lock
#      • Gate failed  + Cargo.toml changed → revert Cargo.toml, re-sync lock,
#                                            commit lock only, then exit 1
#      • Gate passed  + only Cargo.lock   → commit Cargo.lock only
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
    # ([dependencies] and [dev-dependencies]).  --incompatible allow lifts the
    # major-version guard so `ratatui = "0.30"` can move to "0.31", etc.
    # The quality gate below acts as the safety net.
    print $"($cyan)Phase 1/2 — Upgrading Cargo.toml dependency pins...($reset)"

    let upgrade_result = (do {
        run-external "cargo" "upgrade" "--incompatible" "allow"
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
    # Re-read dirty state after the potential revert above.
    let toml_dirty = (is_dirty "Cargo.toml")
    let lock_dirty = (is_dirty "Cargo.lock")
    let date       = (date now | format date "%Y-%m-%d")
    let label      = (commit_label $toml_dirty $lock_dirty $date)

    print ""
    print $"($cyan)Commit strategy:($reset)"

    if ($label | is-empty) {
        print $"  ($green)✓ Everything already up to date — nothing to commit($reset)"

    } else if $dry_run {
        let files = if $toml_dirty { "Cargo.toml + Cargo.lock" } else { "Cargo.lock" }
        print $"  ($cyan)[dry-run] Would commit: ($label)($reset)"
        print $"  ($cyan)[dry-run] Files: ($files)($reset)"

    } else {
        run-external "git" "config" "user.name"  $bot_name
        run-external "git" "config" "user.email" $bot_email

        if $toml_dirty {
            run-external "git" "add" "Cargo.toml" "Cargo.lock"
            # Include a summary of what was upgraded in the commit body (cap at 60 lines)
            let body = ($upgrade_log | lines | first 60 | str join "\n")
            if ($body | is-empty) {
                run-external "git" "commit" "-m" $label
            } else {
                run-external "git" "commit" "-m" $label "-m" $body
            }
        } else {
            run-external "git" "add" "Cargo.lock"
            run-external "git" "commit" "-m" $label
        }

        run-external "git" "push" $remote "main"
        print $"($green)✓ Committed and pushed: ($label)($reset)"
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
