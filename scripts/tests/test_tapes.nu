#!/usr/bin/env nu
# Asserts VHS tapes never show compilation: 'cargo run' only between Hide and Show.
use std/assert
use runner.nu *

def "test tapes hide cargo run" [] {
    let dir = ($env.CURRENT_FILE | path dirname | path dirname | path dirname | path join "examples" "vhs")
    for tape in (glob ($dir | path join "*.tape")) {
        mut hidden = false
        mut seen = false
        for line in (open --raw $tape | lines) {
            let l = ($line | str trim)
            if $l == "Hide" { $hidden = true }
            if $l == "Show" { $hidden = false }
            if ($l | str contains "cargo run") {
                $seen = true
                assert $hidden $"($tape): cargo run visible"
            }
        }
        assert $seen $"($tape): no cargo run"
    }
}

def main [] {
    print $"(ansi cyan)═══ test_tapes.nu ═══(ansi reset)"
    run-tests
}
