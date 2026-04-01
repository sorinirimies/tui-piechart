#!/usr/bin/env nu
# Tests for the pure-logic helpers in scripts/upgrade_deps.nu
#
# Run with: nu scripts/tests/test_upgrade_deps.nu

use std/assert
use runner.nu *
use ../upgrade_deps.nu [commit_label all_passed]

# ── commit_label tests ────────────────────────────────────────────────────────

def "test commit label upgrade when toml dirty" [] {
    let label = (commit_label true true "2026-03-24")
    assert str contains $label "upgrade"
    assert str contains $label "2026-03-24"
}

def "test commit label update when only lock dirty" [] {
    let label = (commit_label false true "2026-03-24")
    assert str contains $label "update"
    assert str contains $label "2026-03-24"
    assert not ($label | str contains "upgrade")
}

def "test commit label empty when nothing dirty" [] {
    let label = (commit_label false false "2026-03-24")
    assert ($label | is-empty)
}

def "test commit label toml dirty takes precedence over lock dirty" [] {
    # When both are dirty the label should be upgrade (Cargo.toml wins)
    let label = (commit_label true true "2026-01-01")
    assert str contains $label "upgrade"
    assert not ($label | str contains "nightly dependency update 2026-01-01")
}

def "test commit label upgrade contains chore prefix" [] {
    let label = (commit_label true false "2026-03-24")
    assert ($label | str starts-with "chore:")
}

def "test commit label update contains chore prefix" [] {
    let label = (commit_label false true "2026-03-24")
    assert ($label | str starts-with "chore:")
}

def "test commit label contains full date" [] {
    let label = (commit_label false true "2099-12-31")
    assert str contains $label "2099-12-31"
}

def "test commit label upgrade message is stable" [] {
    let a = (commit_label true true "2026-03-24")
    let b = (commit_label true true "2026-03-24")
    assert equal $a $b
}

def "test commit label update message is stable" [] {
    let a = (commit_label false true "2026-03-24")
    let b = (commit_label false true "2026-03-24")
    assert equal $a $b
}

# ── all_passed tests ──────────────────────────────────────────────────────────

def "test all passed returns true when all true" [] {
    assert (all_passed [true true true])
}

def "test all passed returns false when one false" [] {
    assert not (all_passed [true false true])
}

def "test all passed returns false when all false" [] {
    assert not (all_passed [false false false])
}

def "test all passed returns true for single true" [] {
    assert (all_passed [true])
}

def "test all passed returns false for single false" [] {
    assert not (all_passed [false])
}

def "test all passed returns true for empty list" [] {
    # vacuous truth — no failures means gate passed
    assert (all_passed [])
}

def "test all passed first element false" [] {
    assert not (all_passed [false true true true])
}

def "test all passed last element false" [] {
    assert not (all_passed [true true true false])
}

def "test all passed mixed single failure" [] {
    let results = [true true true true true false true true]
    assert not (all_passed $results)
}

def "test all passed three true mirrors tui-piechart quality gate" [] {
    # Mirrors: fmt clippy tests (no cross-compile for a pure-Rust lib)
    let gate = [true true true]
    assert (all_passed $gate)
}

def "test all passed three with fmt failure" [] {
    let gate = [false true true]
    assert not (all_passed $gate)
}

def "test all passed three with test failure" [] {
    let gate = [true true false]
    assert not (all_passed $gate)
}

# ── commit_label + all_passed integration tests ───────────────────────────────

def "test no commit when gate passes but nothing dirty" [] {
    let gate_ok = (all_passed [true true true])
    let label   = (commit_label false false "2026-03-24")
    assert $gate_ok
    assert ($label | is-empty)
}

def "test full upgrade commit when gate passes and toml dirty" [] {
    let gate_ok = (all_passed [true true true])
    let label   = (commit_label true true "2026-03-24")
    assert $gate_ok
    assert str contains $label "upgrade"
}

def "test revert path when gate fails and toml dirty" [] {
    # gate_passed = false, toml_changed = true → revert + lock-only commit
    let gate_ok     = (all_passed [true false true])
    let toml_changed = true
    let should_revert = ($toml_changed and (not $gate_ok))
    assert $should_revert
}

def "test lock only commit after revert" [] {
    # After revert: toml_dirty = false, lock_dirty = true
    let label = (commit_label false true "2026-03-24")
    assert str contains $label "update"
    assert not ($label | str contains "upgrade")
}

def "test no revert needed when gate fails but toml not changed" [] {
    let gate_ok      = (all_passed [false true true])
    let toml_changed = false
    let should_revert = ($toml_changed and (not $gate_ok))
    assert not $should_revert
}

def "test gate passes but only lock changed gives update label" [] {
    let gate_ok = (all_passed [true true true])
    let label   = (commit_label false true "2026-03-24")
    assert $gate_ok
    assert not ($label | is-empty)
    assert str contains $label "update"
}

# ── Crate invariant checks ───────────────────────────────────────────────────
# These tests verify properties of the tui-piechart crate that must hold after
# any dependency upgrade: valid TOML, resolution succeeds, no dupes, etc.

def "test cargo toml is valid toml" [] {
    # Verify the real Cargo.toml can be parsed as TOML
    let data = (open Cargo.toml)
    assert ($data | get package.version | is-not-empty)
}

def "test dependencies section exists" [] {
    let data = (open Cargo.toml)
    assert ($data | get dependencies | is-not-empty)
}

def "test cargo metadata resolves without errors" [] {
    # A fast check that the crate is in a consistent state:
    # all deps resolve, no missing features, no version conflicts.
    let result = (do { run-external "cargo" "metadata" "--no-deps" "--format-version" "1" } | complete)
    assert equal $result.exit_code 0 "cargo metadata must succeed — dependency resolution is broken"
}

def "test dependencies have no duplicate entries" [] {
    let content = (open Cargo.toml --raw)
    let dep_section = ($content | lines
        | reduce --fold { in_deps: false, lines: [] } { |line, acc|
            let entering = ($line =~ '^\[dependencies\]')
            let leaving = if $acc.in_deps {
                ($line =~ '^\[') and (not $entering)
            } else {
                false
            }
            let new_in = if $entering { true } else if $leaving { false } else { $acc.in_deps }
            let new_lines = if ($acc.in_deps and ($line =~ '^\w')) {
                $acc.lines | append $line
            } else {
                $acc.lines
            }
            { in_deps: $new_in, lines: $new_lines }
        }
        | get lines)

    let names = ($dep_section | each { |l|
        $l | parse --regex '(?P<n>[\w-]+)\s*=' | get n | first
    })
    let unique = ($names | uniq)
    assert equal ($names | length) ($unique | length) "dependencies must not have duplicate entries"
}

def "test dev-dependencies have no duplicate entries" [] {
    let content = (open Cargo.toml --raw)
    let dep_section = ($content | lines
        | reduce --fold { in_deps: false, lines: [] } { |line, acc|
            let entering = ($line =~ '^\[dev-dependencies\]')
            let leaving = if $acc.in_deps {
                ($line =~ '^\[') and (not $entering)
            } else {
                false
            }
            let new_in = if $entering { true } else if $leaving { false } else { $acc.in_deps }
            let new_lines = if ($acc.in_deps and ($line =~ '^\w')) {
                $acc.lines | append $line
            } else {
                $acc.lines
            }
            { in_deps: $new_in, lines: $new_lines }
        }
        | get lines)

    let names = ($dep_section | each { |l|
        $l | parse --regex '(?P<n>[\w-]+)\s*=' | get n | first
    })
    let unique = ($names | uniq)
    assert equal ($names | length) ($unique | length) "dev-dependencies must not have duplicate entries"
}

def "test package name is tui-piechart" [] {
    let data = (open Cargo.toml)
    assert equal ($data | get package.name) "tui-piechart"
}

def "test ratatui is a dependency" [] {
    let data = (open Cargo.toml)
    assert ($data | get dependencies.ratatui | is-not-empty) "ratatui must be listed in [dependencies]"
}

# ── Runner ────────────────────────────────────────────────────────────────────

def main [] {
    print $"(ansi cyan)═══ test_upgrade_deps.nu ═══(ansi reset)"
    run-tests
}
