#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Fuzz every cargo-fuzz target for a while (nightly toolchain)
# ──────────────────────────────────────────────────────────────────────────────
# New targets are picked up automatically (cargo fuzz list). Stops at the first
# crash; the reproducer is in fuzz/artifacts/<target>/.
#
# cargo-fuzz needs nightly, and a nightly regression in any dependency would stop
# fuzzing altogether. So when the latest nightly cannot *build* the targets, the
# run falls back to an older nightly (FALLBACK_DAYS ago) and says so. A crash
# while fuzzing still fails the run.
#
# Usage: nu scripts/ci/fuzz_all.nu [--secs 60]
# ──────────────────────────────────────────────────────────────────────────────

# How many days back the fallback nightlies are.
export const FALLBACK_DAYS = [7 14]

# The toolchains to try, newest first: nightly, then nightly-YYYY-MM-DD fallbacks.
export def toolchains [today: datetime] {
    ["nightly" ...($FALLBACK_DAYS | each {|d| $"nightly-(($today - ($d * 1day)) | format date '%Y-%m-%d')" })]
}

export def fuzz_args [toolchain: string, target: string, secs: int] {
    [$"+($toolchain)" fuzz run $target -- $"-max_total_time=($secs)"]
}

# The first toolchain that builds every fuzz target, installing dated ones on demand.
def pick_toolchain [] {
    for tc in (toolchains (date now)) {
        if $tc != "nightly" {
            print -e $"(ansi yellow)⚠(ansi reset) falling back to ($tc)"
            ^rustup toolchain install $tc --profile minimal --no-self-update
        }
        let built = ^cargo $"+($tc)" fuzz build | complete
        if $built.exit_code == 0 { return $tc }
        print -e ($built.stderr | lines | where {|l| $l =~ '^error' } | first 5 | str join "\n")
    }
    print -e $"(ansi red)✗(ansi reset) no nightly in (toolchains (date now)) builds the fuzz targets"
    exit 1
}

def main [--secs: int = 60] {
    let tc = pick_toolchain
    print $"toolchain=($tc)"
    let targets = ^cargo $"+($tc)" fuzz list | lines | where {|l| $l | is-not-empty }
    if ($targets | is-empty) { print -e "no fuzz targets"; exit 1 }
    for t in $targets {
        print -e $"(ansi cyan)▶(ansi reset) ($t) for ($secs)s on ($tc)"
        ^cargo ...(fuzz_args $tc $t $secs)
    }
}
