#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# Nightly failure notice (used by the notify-failure job in deps-update.yml)
# ──────────────────────────────────────────────────────────────────────────────
#   nu scripts/ci/notify_failure.nu   open one issue for a failing nightly, or
#                                     comment on it while it stays red
# Needs GH_TOKEN and the GITHUB_* variables of an Actions run.
# ──────────────────────────────────────────────────────────────────────────────

export const TITLE = "Nightly dependency update is failing"

export def run_url [server: string, repo: string, run: string]: nothing -> string {
    $"($server)/($repo)/actions/runs/($run)"
}

export def issue_body [url: string]: nothing -> string {
    $"The scheduled run failed: ($url)\n\nIf the log says `could not read Username`, the GH_PAT secret is expired or invalid."
}

def main [] {
    let repo = $env.GITHUB_REPOSITORY
    let url = run_url $env.GITHUB_SERVER_URL $repo $env.GITHUB_RUN_ID
    let open = ^gh issue list --repo $repo --state open --search $"($TITLE) in:title" --json number | from json
    if ($open | is-empty) {
        ^gh issue create --repo $repo --title $TITLE --body (issue_body $url)
    } else {
        ^gh issue comment ($open.0.number | into string) --repo $repo --body $"Failed again: ($url)"
    }
}
