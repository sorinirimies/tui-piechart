#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — One git operation on every remote
# ──────────────────────────────────────────────────────────────────────────────
# GitHub (origin) and Gitea (gitea-starscream) are independent hosts, no mirroring.
# Unconfigured remotes are skipped; a failing remote doesn't stop the others, but
# the run fails at the end.
#
# Usage:
#   nu scripts/remotes.nu push [--force]     current branch + tags
#   nu scripts/remotes.nu push-tags
#   nu scripts/remotes.nu pull
#   nu scripts/remotes.nu setup              add the missing remotes
# ──────────────────────────────────────────────────────────────────────────────

export const REMOTES = {
    origin: "git@github.com:sorinirimies/tui-piechart.git"
    gitea-starscream: "gitea@192.168.1.44:sorin/tui-piechart.git"
}

# The wanted remotes that are configured, in REMOTES order.
export def targets [configured: list<string>] {
    $REMOTES | columns | where {|r| $r in $configured }
}

# The git arguments for one remote.
export def git_args [op: string, remote: string, branch: string, --force] {
    match $op {
        "push" => ([push --follow-tags] | append (if $force { [--force] } else { [] }) | append [$remote $branch])
        "push-tags" => [push $remote --tags]
        "pull" => [pull $remote $branch]
        _ => (error make { msg: $"unknown operation ($op)" })
    }
}

def on_all [op: string, --force] {
    let configured = ^git remote | lines
    let branch = ^git branch --show-current | str trim
    for r in ($REMOTES | columns | where {|r| $r not-in $configured }) { print $"↷ skipping ($r) \(not configured\)" }
    let failed = targets $configured | where {|r|
        print $"(ansi cyan)▶(ansi reset) git (git_args $op $r $branch --force=$force | str join ' ')"
        try { ^git ...(git_args $op $r $branch --force=$force); false } catch { true }
    }
    if ($failed | is-not-empty) {
        print $"⚠️  failed: ($failed | str join ', ')"
        exit 1
    }
}

def "main push" [--force] { on_all push --force=$force }
def "main push-tags" [] { on_all push-tags }
def "main pull" [] { on_all pull }

def "main setup" [] {
    let configured = ^git remote | lines
    for r in ($REMOTES | transpose name url | where {|r| $r.name not-in $configured }) {
        ^git remote add $r.name $r.url
        print $"+ ($r.name) → ($r.url)"
    }
    ^git remote -v
}

def main [] { print "usage: nu scripts/remotes.nu (push [--force] | push-tags | pull | setup)" }
