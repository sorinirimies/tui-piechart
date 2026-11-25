#!/usr/bin/env nu
# Tests for scripts/ci/notify_failure.nu

use std/assert
use runner.nu *
use ../ci/notify_failure.nu [run_url, issue_body, TITLE]

def "test notify_failure: the run url points at the failing run" [] {
    assert equal (run_url "https://github.com" "o/r" "42") "https://github.com/o/r/actions/runs/42"
}

def "test notify_failure: the body links the run and names the token hint" [] {
    let body = issue_body "https://x/run/1"
    assert ($body | str contains "https://x/run/1")
    assert ($body | str contains "GH_PAT")
}

def "test notify_failure: one fixed title, so a red streak shares an issue" [] {
    assert equal $TITLE "Nightly dependency update is failing"
}

def main [] { run-tests }
