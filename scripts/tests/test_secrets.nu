use std/assert
use runner.nu *

def root [] { $env.CURRENT_FILE | path dirname | path join ".." ".." }

def "test secrets: both hosts scan the full history with gitleaks" [] {
    for host in [".github" ".gitea"] {
        let wf = open (root | path join $host "workflows" "secrets.yml")
        let steps = $wf.jobs.gitleaks.steps
        let checkout = $steps | where {|s| ($s.uses? | default "") | str starts-with "actions/checkout@" } | first
        assert equal $checkout.with.fetch-depth 0 $"($host): gitleaks needs the whole history"
        assert ($steps | any {|s| ($s.run? | default "") =~ "sha256sum -c" }) $"($host): the gitleaks download is not checksummed"
        assert ($steps | any {|s| ($s.run? | default "") | str starts-with "gitleaks git --redact" }) $"($host): no redacted gitleaks scan"
    }
}

def "test secrets: the scan runs on every push, not only on dependency changes" [] {
    for host in [".github" ".gitea"] {
        let on = (open (root | path join $host "workflows" "secrets.yml")).on
        assert ($on.push.paths? | is-empty) $"($host): push is limited to some paths"
    }
}

def "test secrets: the justfile scans locally like CI" [] {
    let j = open --raw (root | path join "justfile") | decode utf-8
    assert ($j | str contains "secrets:\n    gitleaks git --redact") "just secrets is missing"
}

def main [] { run-tests }
