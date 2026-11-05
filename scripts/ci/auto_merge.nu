#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Merge Dependabot PRs once CI is green (auto-merge.yml, deps-update.yml)
# ──────────────────────────────────────────────────────────────────────────────
#   nu scripts/ci/auto_merge.nu <branch> <sha>   after CI: merge the Dependabot PR on
#                                                <branch> if its head is still <sha>
#   nu scripts/ci/auto_merge.nu sweep            nightly: merge every open Dependabot
#                                                PR whose checks all passed; ask
#                                                Dependabot to rebase conflicting ones
#
# Dependabot only bumps GitHub Actions here (.github/dependabot.yml), majors
# included: the Gitea nightly (actions_update.nu) takes majors unattended too, and
# CI passing on the PR shows the workflows still run. Prints merged=<numbers>.
# GITHUB_TOKEN cannot merge a PR that changes .github/workflows/*: pass GH_PAT.
# ──────────────────────────────────────────────────────────────────────────────

const DEPENDABOT = "app/dependabot"

# Whether to merge `pr` (`gh pr list --json number,author,headRefOid,state`) whose CI
# run was for `sha`: `{merge: bool, reason: string}`.
export def decide [pr: record, sha: string]: nothing -> record {
    if $pr.author.login != $DEPENDABOT {
        return {merge: false, reason: $"not a Dependabot PR \(($pr.author.login))"}
    }
    if $pr.state != "OPEN" { return {merge: false, reason: $"PR is ($pr.state)"} }
    if $pr.headRefOid != $sha {
        return {merge: false, reason: "the branch moved since CI ran; the next CI run decides"}
    }
    {merge: true, reason: "CI passed"}
}

# Whether every check in a `statusCheckRollup` passed (skipped and neutral count as
# passed); an empty rollup has not run yet.
export def checks_green [rollup: list]: nothing -> bool {
    let ok = [SUCCESS SKIPPED NEUTRAL]
    ($rollup | is-not-empty) and ($rollup | all {|c| ($c.conclusion? | default ($c.state? | default "")) in $ok })
}

# What the nightly sweep does with an open PR (`gh pr list --json ...,mergeable,
# statusCheckRollup`): "merge", "rebase" (conflicts: Dependabot rebases, or closes
# a bump that no longer applies) or "wait" (checks pending or failed: a human looks).
export def sweep_action [pr: record]: nothing -> string {
    if not (decide $pr $pr.headRefOid).merge { return "wait" }
    if $pr.mergeable == "CONFLICTING" { return "rebase" }
    if (checks_green $pr.statusCheckRollup) { "merge" } else { "wait" }
}

def merge [number: int] { ^gh pr merge $number --squash --delete-branch }

def rebase [number: int] { ^gh pr comment $number --body "@dependabot rebase" | ignore }

def main [branch: string, sha: string] {
    let prs = ^gh pr list --head $branch --state open --json number,author,headRefOid,state | from json
    if ($prs | is-empty) {
        print -e $"no open PR for ($branch)"
        print "merged="
        return
    }
    let pr = $prs | first
    let d = decide $pr $sha
    print -e $"#($pr.number): ($d.reason)"
    if $d.merge { merge $pr.number }
    print $"merged=(if $d.merge { $pr.number } else { '' })"
}

def "main sweep" [] {
    let prs = ^gh pr list --author $DEPENDABOT --state open --json number,title,author,headRefOid,state,mergeable,statusCheckRollup | from json
    mut merged = []
    mut rebased = []
    # One PR failing to merge must not stop the others.
    for p in $prs {
        let action = sweep_action $p
        print -e $"#($p.number) ($p.title): ($action)"
        if $action == "merge" {
            let ok = try { merge $p.number; true } catch { false }
            if $ok {
                $merged = $merged ++ [$p.number]
            } else {
                rebase $p.number
                $rebased = $rebased ++ [$p.number]
            }
        } else if $action == "rebase" {
            rebase $p.number
            $rebased = $rebased ++ [$p.number]
        }
    }
    print $"merged=($merged | str join ',')"
    print $"rebased=($rebased | str join ',')"
}
