#!/usr/bin/env nu
# ── tui-piechart · test_dev_scripts.nu ──────────────────────────────────
# The justfile's helpers: remotes, tools, Gitea dispatch, fuzzing.

use std/assert
use runner.nu *
use ../remotes.nu [REMOTES targets git_args]
use ../tools.nu [TOOLS TARGETS missing install_args]
use ../gitea_run.nu [dispatch_url]
use ../ci/fuzz_all.nu [fuzz_args toolchains FALLBACK_DAYS]

def "test remotes: both hosts, github first" [] {
    assert equal ($REMOTES | columns) ["origin" "gitea-starscream"]
}

def "test remotes: unconfigured remotes are skipped" [] {
    assert equal (targets ["gitea-starscream" "upstream"]) ["gitea-starscream"]
    assert equal (targets []) []
}

def "test remotes: push carries tags and force only when asked" [] {
    assert equal (git_args push origin main) [push --follow-tags origin main]
    assert equal (git_args push origin main --force) [push --follow-tags --force origin main]
    assert equal (git_args push-tags origin main) [push origin --tags]
    assert equal (git_args pull origin main) [pull origin main]
}

def "test tools: only the missing ones are installed" [] {
    assert equal (missing ($TOOLS | columns)) []
    assert ("git-cliff" in (missing []))
}

def "test tools: install args use the crate name and the lockfile" [] {
    assert equal (install_args cargo-machete) [install cargo-machete --locked]
    assert equal (install_args some-tool) [install some-tool --locked]
}

def "test tools: the gate tools are known" [] {
    assert ("cargo-machete" in ($TOOLS | columns))
}

def "test gitea_run: dispatch url" [] {
    let url = dispatch_url "http://h:3000/" "fuzz"
    assert ($url | str starts-with "http://h:3000/api/v1/repos/")
    assert ($url | str ends-with "/actions/workflows/fuzz.yml/dispatches")
    assert equal (dispatch_url "http://h" "ci.yml") (dispatch_url "http://h" "ci")
}

def "test fuzz_all: the chosen toolchain with a time limit" [] {
    assert equal (fuzz_args nightly parse 30) [+nightly fuzz run parse -- -max_total_time=30]
    assert equal (fuzz_args nightly-2026-10-25 parse 30) [+nightly-2026-10-25 fuzz run parse -- -max_total_time=30]
}

def "test fuzz_all: latest nightly first then dated fallbacks" [] {
    let tcs = toolchains ("2026-11-01T12:00:00+00:00" | into datetime)
    assert equal $tcs [nightly nightly-2026-10-25 nightly-2026-10-18]
    assert equal ($tcs | length) (($FALLBACK_DAYS | length) + 1)
}

def main [] { run-tests }

def "test tools: targets are rustup target triples" [] {
    for t in $TARGETS { assert ($t =~ '^[a-z0-9_]+(-[a-z0-9_]+){1,3}$') $"($t) is a target triple" }
}
