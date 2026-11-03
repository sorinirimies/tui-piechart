#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Start a Gitea workflow now (workflow_dispatch)
# ──────────────────────────────────────────────────────────────────────────────
# Env: GITEA_URL (e.g. http://192.168.1.44:3000), GITEA_TOKEN (repo scope).
#
# Usage: nu scripts/gitea_run.nu <workflow>     e.g. deps-update | security | fuzz
# ──────────────────────────────────────────────────────────────────────────────

export const REPO = "sorin/tui-piechart"

export def dispatch_url [base: string, workflow: string] {
    let file = if ($workflow | str ends-with ".yml") { $workflow } else { $"($workflow).yml" }
    $"($base | str trim --right --char '/')/api/v1/repos/($REPO)/actions/workflows/($file)/dispatches"
}

def main [workflow: string] {
    let base = $env.GITEA_URL? | default ""
    let token = $env.GITEA_TOKEN? | default ""
    if ($base | is-empty) or ($token | is-empty) {
        print "❌ set GITEA_URL (e.g. http://192.168.1.44:3000) and GITEA_TOKEN"
        exit 1
    }
    http post --content-type application/json --headers { Authorization: $"token ($token)" } (dispatch_url $base $workflow) { ref: "main" } | ignore
    print $"✅ dispatched ($workflow) on Gitea"
}
