#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Bump `uses: owner/repo@vN` in .gitea/workflows (Gitea has no Dependabot)
# ──────────────────────────────────────────────────────────────────────────────
# Looks up each action's latest major on GitHub and rewrites older majors. Prints
# changed=true|false. actions/checkout and actions/*-artifact stay pinned: act_runner
# lacks the newer node / artifact backends they need.
#
# Usage: nu scripts/ci/actions_update.nu [--dir .gitea/workflows]
# ──────────────────────────────────────────────────────────────────────────────

export const PINNED = ["actions/checkout" "actions/upload-artifact" "actions/download-artifact"]

# Every `owner/repo` → major (int) used in a workflow text.
export def uses [text: string] {
    $text
    | parse --regex 'uses:\s*(?<repo>[\w.-]+/[\w.-]+)@v(?<major>\d+)'
    | update major { into int }
}

# `v5.2.0` → 5; anything else → null.
export def major_of [tag: string] {
    let m = $tag | parse --regex '^v(?<n>\d+)'
    if ($m | is-empty) { null } else { $m.0.n | into int }
}

# Rewrite every `repo@vN` with N < latest.repo; pinned and unknown repos are untouched.
export def bump [text: string, latest: record] {
    uses $text
    | where {|u| $u.repo not-in $PINNED and ($latest | get -o $u.repo) != null and $u.major < ($latest | get $u.repo) }
    | uniq
    | reduce --fold $text {|u, acc|
        $acc | str replace --all --regex $"($u.repo | str replace --all '.' '\.')@v($u.major)\\b" $"($u.repo)@v($latest | get $u.repo)"
    }
}

def latest_major [repo: string] {
    let r = try { http get $"https://api.github.com/repos/($repo)/releases/latest" } catch { null }
    if $r == null { null } else { major_of $r.tag_name }
}

def "main commit" [bot: string, email: string, --dir: path = ".gitea/workflows"] {
    ^git config user.name $bot
    ^git config user.email $email
    ^git add $dir
    ^git commit -m "ci(deps): bump workflow actions"
    ^git pull --rebase origin main
    ^git push origin HEAD:main
}

def main [--dir: path = ".gitea/workflows"] {
    let files = glob $"($dir)/*.yml"
    let repos = $files | each {|f| uses (open --raw $f) | get repo } | flatten | uniq | where {|r| $r not-in $PINNED }
    let latest = $repos | reduce --fold {} {|r, acc|
        let m = latest_major $r
        if $m == null { $acc } else { $acc | insert $r $m }
    }
    mut changed = false
    for f in $files {
        let before = open --raw $f
        let after = bump $before $latest
        if $after != $before { $after | save --force --raw $f; print -e $"bumped ($f)"; $changed = true }
    }
    print $"changed=($changed)"
}
