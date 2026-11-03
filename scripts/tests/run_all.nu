#!/usr/bin/env nu
# ── tui-piechart · Run All Nushell Tests ────────────────────────────────────────
#
# Usage:
#   nu scripts/tests/run_all.nu          # run every test_*.nu file
#   nu scripts/tests/run_all.nu --filter version   # only files matching "version"

def main [--filter: string = ""] {
    let green  = (ansi green)
    let red    = (ansi red)
    let bold   = (ansi --escape {attr: b})
    let cyan   = (ansi cyan)
    let reset  = (ansi reset)

    print $"($bold)($cyan)── tui-piechart · Nushell Test Suite ──($reset)"
    print ""

    let test_dir = ($env.CURRENT_FILE | path dirname)

    let files = (
        ls $test_dir
        | where { |it| ($it.name | path basename) =~ '^test_.*\.nu$' }
        | get name
        | sort
    )

    let files = if $filter == "" {
        $files
    } else {
        $files | where { |f| ($f | path basename) =~ $filter }
    }

    let total = ($files | length)

    if $total == 0 {
        print $"($red)No test files found.(ansi reset)"
        exit 1
    }

    print $"Found ($total) test file\(s\):"
    for f in $files {
        print $"  ($cyan)($f | path basename)($reset)"
    }
    print ""

    mut passed = 0
    mut failed = 0
    mut failures = []

    for f in $files {
        let name = ($f | path basename)
        print $"($bold)▶ ($name)($reset)"
        let res = (do { nu $f } | complete)

        # Forward the test file's stdout so the user sees per-test results.
        if ($res.stdout | str trim) != "" {
            print $res.stdout
        }

        if $res.exit_code == 0 {
            $passed = $passed + 1
        } else {
            $failed = $failed + 1
            $failures = ($failures | append $name)
            if ($res.stderr | str trim) != "" {
                print $res.stderr
            }
        }
        print ""
    }

    print $"($bold)── Summary ──($reset)"
    print $"  ($green)($passed) passed($reset) · ($red)($failed) failed($reset) · ($total) total"

    if $failed > 0 {
        print ""
        print $"($red)($bold)Failed files:($reset)"
        for f in $failures {
            print $"  ($red)✗($reset) ($f)"
        }
        exit 1
    }

    print ""
    print $"($green)($bold)✓ All ($total) test files passed!($reset)"
}
