#!/usr/bin/env nu
# Pre-publish readiness check for tui-piechart.
# Usage: nu scripts/check_publish.nu
# Run this before pushing a release tag to catch problems early.

# ── Helpers ───────────────────────────────────────────────────────────────────

# Check whether all required files exist in the current directory.
# Returns a list of missing file names.
export def check_required_files [files: list<string>] {
    $files | where { |f| not ($f | path exists) }
}

# Count the number of failed checks (false values) in a list of booleans.
export def count_errors [checks: list<bool>] {
    $checks | where { |c| not $c } | length
}

# ── Main ──────────────────────────────────────────────────────────────────────

def main [] {
    let green  = (ansi green)
    let red    = (ansi red)
    let yellow = (ansi yellow)
    let cyan   = (ansi cyan)
    let reset  = (ansi reset)

    print ""
    print $"($cyan)════════════════════════════════════════($reset)"
    print $"($cyan)  tui-piechart — Publish Readiness Check($reset)"
    print $"($cyan)════════════════════════════════════════($reset)"
    print ""

    mut errors = 0

    # ── 1. Formatting ─────────────────────────────────────────────────────────
    print $"($cyan)── Formatting ──($reset)"
    print -n "  cargo fmt -- --check ... "
    let fmt = (do { run-external "cargo" "fmt" "--" "--check" } | complete)
    if $fmt.exit_code == 0 {
        print $"($green)✓ code is formatted($reset)"
    } else {
        print $"($red)✗ formatting issues found  \(run: cargo fmt\)($reset)"
        $errors = $errors + 1
    }

    # ── 2. Clippy ─────────────────────────────────────────────────────────────
    print ""
    print $"($cyan)── Clippy ──($reset)"
    print -n "  cargo clippy --lib -- -D warnings ... "
    let clippy = (do {
        run-external "cargo" "clippy" "--lib" "--" "-D" "warnings"
    } | complete)
    if $clippy.exit_code == 0 {
        print $"($green)✓ no clippy warnings($reset)"
    } else {
        print $"($red)✗ clippy found issues  \(run: cargo clippy --lib -- -D warnings\)($reset)"
        $errors = $errors + 1
    }

    # ── 3. Tests ──────────────────────────────────────────────────────────────
    print ""
    print $"($cyan)── Tests ──($reset)"
    print -n "  cargo test --all-features ... "
    let tests = (do {
        run-external "cargo" "test" "--all-features"
    } | complete)
    if $tests.exit_code == 0 {
        print $"($green)✓ all tests pass($reset)"
    } else {
        print $"($red)✗ test failures found  \(run: cargo test --all-features\)($reset)"
        $errors = $errors + 1
    }

    # ── 4. Documentation ──────────────────────────────────────────────────────
    print ""
    print $"($cyan)── Documentation ──($reset)"
    print -n "  cargo doc --no-deps --all-features ... "
    let doc = (do {
        run-external "cargo" "doc" "--no-deps" "--all-features"
    } | complete)
    if $doc.exit_code == 0 {
        print $"($green)✓ documentation builds($reset)"
    } else {
        print $"($red)✗ documentation build failed  \(run: cargo doc --no-deps\)($reset)"
        $errors = $errors + 1
    }

    # ── 5. Examples ───────────────────────────────────────────────────────────
    print ""
    print $"($cyan)── Examples ──($reset)"
    print -n "  cargo build --examples ... "
    let examples = (do {
        run-external "cargo" "build" "--examples"
    } | complete)
    if $examples.exit_code == 0 {
        print $"($green)✓ all examples build($reset)"
    } else {
        print $"($red)✗ example build failed  \(run: cargo build --examples\)($reset)"
        $errors = $errors + 1
    }

    # ── 6. Required files ─────────────────────────────────────────────────────
    print ""
    print $"($cyan)── Required files ──($reset)"
    let required = ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md"]
    let missing = (check_required_files $required)
    if ($missing | is-empty) {
        for f in $required {
            print $"  ($f) ... ($green)✓ present($reset)"
        }
    } else {
        for f in $required {
            if ($f | path exists) {
                print $"  ($f) ... ($green)✓ present($reset)"
            } else {
                print $"  ($f) ... ($red)✗ missing($reset)"
                $errors = $errors + 1
            }
        }
    }

    # ── 7. Cargo.lock ─────────────────────────────────────────────────────────
    print ""
    print $"($cyan)── Cargo.lock ──($reset)"
    print -n "  Cargo.lock present ... "
    if ("Cargo.lock" | path exists) {
        print $"($green)✓ present($reset)"
    } else {
        print $"($red)✗ missing — run: cargo generate-lockfile($reset)"
        $errors = $errors + 1
    }

    # ── 8. Publish readiness ──────────────────────────────────────────────────
    print ""
    print $"($cyan)── Publish readiness ──($reset)"
    print -n "  cargo publish --dry-run ... "
    let dry = (do {
        run-external "cargo" "publish" "--dry-run" "--allow-dirty"
    } | complete)
    if $dry.exit_code == 0 {
        print $"($green)✓ publish dry-run passed($reset)"
    } else {
        print $"($red)✗ publish dry-run failed  \(run: cargo publish --dry-run --allow-dirty\)($reset)"
        $errors = $errors + 1
    }

    # ── Summary ───────────────────────────────────────────────────────────────
    print ""
    print $"($cyan)════════════════════════════════════════($reset)"
    if $errors == 0 {
        print $"($green)✓ All checks passed — ready to publish! 🚀($reset)"
        print ""
        print $"($cyan)Next step:($reset)"
        print "  just bump <version>   # e.g. just bump 0.3.2"
    } else {
        let plural = if $errors == 1 { "check" } else { "checks" }
        print $"($red)✗ ($errors) ($plural) failed — please fix before publishing.($reset)"
        exit 1
    }
}
