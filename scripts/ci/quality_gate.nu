#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Quality gate
# ──────────────────────────────────────────────────────────────────────────────
# The same sequence locally (`just check-all`), in CI, in the nightly dependency
# update and before a bump:
#   1. cargo fmt --check
#   2. cargo clippy (workspace, all targets, -D warnings)
#   3. cargo test --workspace
#   4. cargo doc with warnings as errors (broken intra-doc links fail docs.rs)
#   5. versions agree (Cargo.toml == EXTRA_MANIFESTS)
#   6. nushell script tests
# Project-specific steps (a wasm build, a web bundle, …) go in as extra run_step
# calls — one place, so every caller gets them.
#
# Usage: nu scripts/ci/quality_gate.nu [--skip-fmt] [--skip-test] [--skip-nu]
# ──────────────────────────────────────────────────────────────────────────────

def run_step [label: string, cmd: closure]: nothing -> bool {
    print $"(ansi cyan)▶(ansi reset) ($label)"
    let r = (do $cmd | complete)
    if $r.exit_code == 0 {
        print $"(ansi green)  ✔ ok(ansi reset)"
        true
    } else {
        print $"(ansi red)  ✗ failed(ansi reset)"
        if ($r.stdout | str trim | is-not-empty) { print $r.stdout }
        if ($r.stderr | str trim | is-not-empty) { print $r.stderr }
        false
    }
}

def main [--skip-fmt, --skip-test, --skip-nu] {
    print $"(ansi cyan)══ tui-piechart — Quality Gate ══(ansi reset)"
    mut ok = true
    if not $skip_fmt {
        $ok = (run_step "cargo fmt --all -- --check" { cargo fmt --all -- --check }) and $ok
    }
    $ok = (run_step "cargo clippy --workspace --all-targets -- -D warnings" {
        cargo clippy --workspace --all-targets --all-features -- -D warnings
    }) and $ok
    # Users may turn the default features off: the library alone must still build clean.
    $ok = (run_step "cargo clippy --lib --no-default-features -- -D warnings" {
        cargo clippy --lib --no-default-features -- -D warnings
    }) and $ok
    if not $skip_test {
        $ok = (run_step "cargo test --workspace" { cargo test --workspace --all-features }) and $ok
    }
    $ok = (run_step "cargo doc (warnings are errors)" {
        with-env { RUSTDOCFLAGS: "-D warnings" } { cargo doc --no-deps --all-features --quiet }
    }) and $ok
    $ok = (run_step "versions agree" { nu scripts/ci/check_versions.nu }) and $ok
    if not $skip_nu {
        $ok = (run_step "nushell tests" { nu scripts/tests/run_all.nu }) and $ok
    }
    if not $ok {
        print $"(ansi red)✗ Quality gate FAILED(ansi reset)"
        exit 1
    }
    print $"(ansi green)✔ Quality gate passed(ansi reset)"
}
