#!/usr/bin/env nu
# Build examples first, then record every VHS tape (GIFs show results, never compilation).
# Usage: nu scripts/generate_all_tapes.nu

def main [] {
    let root = ($env.FILE_PWD | path dirname)
    cd $root
    let build = (do { cargo build --release --features theme --examples } | complete)
    if $build.exit_code != 0 {
        print -e $build.stderr
        error make { msg: "cargo build --release --features theme --examples failed" }
    }
    mkdir examples/vhs/output
    for tape in (glob examples/vhs/*.tape | sort) {
        print $"Recording ($tape | path basename) ..."
        vhs $tape
        if $env.LAST_EXIT_CODE != 0 { error make { msg: $"vhs failed for ($tape)" } }
    }
}
