#!/usr/bin/env nu
# Run all Nu test suites in scripts/tests/.
#
# Usage: nu scripts/tests/run_all.nu

def main [] {
    let green  = (ansi green)
    let red    = (ansi red)
    let cyan   = (ansi cyan)
    let bold   = (ansi --escape {attr: b})
    let reset  = (ansi reset)

    let tests_dir = ($env.CURRENT_FILE | path dirname)

    let suites = (
        ls $tests_dir
        | where { |f| ($f.name | path basename) =~ '^test_.*\.nu$' }
        | get name
        | sort
    )

    let total_suites = ($suites | length)

    if $total_suites == 0 {
        print $"($red)No test suites found in ($tests_dir)($reset)"
        exit 1
    }

    print $"($cyan)($bold)════════════════════════════════════════($reset)"
    print $"($cyan)($bold)  tui-piechart — Nu Script Test Runner($reset)"
    print $"($cyan)($bold)════════════════════════════════════════($reset)"
    print $"Found ($total_suites) suite\(s\)"
    print ""

    mut passed_suites  = 0
    mut failed_suites  = 0
    mut suite_results  = []

    for suite in $suites {
        let name = ($suite | path basename)
        print $"($bold)▶ ($name)($reset)"

        let res = (do { nu $suite } | complete)

        # Print the suite's output indented
        for line in ($res.stdout | lines) {
            print $"  ($line)"
        }

        if $res.exit_code == 0 {
            $passed_suites = $passed_suites + 1
            $suite_results = ($suite_results | append { suite: $name, status: "passed" })
        } else {
            # Also print stderr if present
            if not ($res.stderr | is-empty) {
                for line in ($res.stderr | lines) {
                    print $"  ($red)($line)($reset)"
                }
            }
            $failed_suites = $failed_suites + 1
            $suite_results = ($suite_results | append { suite: $name, status: "failed" })
        }

        print ""
    }

    # ── Summary table ─────────────────────────────────────────────────────────
    print $"($cyan)($bold)════════════════════════════════════════($reset)"
    print $"($bold)Summary($reset)"
    print $"($cyan)($bold)════════════════════════════════════════($reset)"

    for r in $suite_results {
        let icon   = if $r.status == "passed" { $"($green)✓($reset)" } else { $"($red)✗($reset)" }
        let status = if $r.status == "passed" { $"($green)passed($reset)" } else { $"($red)failed($reset)" }
        print $"  ($icon) ($r.suite)  ($status)"
    }

    print ""
    print $"Suites: ($green)($passed_suites) passed($reset) · ($red)($failed_suites) failed($reset) · ($total_suites) total suites"

    if $failed_suites > 0 {
        print ""
        print $"($red)($bold)✗ ($failed_suites) suite\(s\) failed.($reset)"
        exit 1
    }

    print ""
    print $"($green)($bold)✓ All ($total_suites) suites passed!($reset)"
}
