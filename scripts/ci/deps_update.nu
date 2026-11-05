#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Nightly dependency update helpers (used by deps-update.yml)
# ──────────────────────────────────────────────────────────────────────────────
#   nu scripts/ci/deps_update.nu sync               reset to origin/main (scheduled runs start stale)
#   nu scripts/ci/deps_update.nu update             cargo update here and in fuzz/
#   nu scripts/ci/deps_update.nu changed            prints changed=true|false (→ $GITHUB_OUTPUT)
#   nu scripts/ci/deps_update.nu commit <bot> <mail> commit the lockfiles, push to main
#   nu scripts/ci/deps_update.nu upgrade            cargo upgrade (compatible pins) + update
#   nu scripts/ci/deps_update.nu release <bot> <mail> commit, bump the patch, push, release
#
# A library's users only see its Cargo.toml requirements, never its lockfile: a
# changed requirement is shipped as a patch release; a lockfile-only change is
# committed and nothing is released.
# ──────────────────────────────────────────────────────────────────────────────

export const LOCKFILES = ["Cargo.lock" "fuzz/Cargo.lock"]

# The lockfiles to commit: those in this checkout (fuzz/ is optional) that git does
# not ignore (`git add` refuses an ignored file).
export def present [files: list<string>, ignored: list<string> = []] {
    $files | where {|f| ($f | path exists) and $f not-in $ignored }
}

# Of `files`, those a .gitignore excludes.
def ignored [files: list<string>]: nothing -> list<string> {
    ^git check-ignore ...$files | complete | get stdout | lines
}


export def commit_message [date: string] {
    $"chore\(deps\): nightly cargo update ($date)"
}

const KINDS = ["dependencies" "dev-dependencies" "build-dependencies"]

# Every dependency requirement of a parsed Cargo.toml, as `kind name` → version
# (`target.<cfg>.<kind>` included). Formatting and key order don't matter.
export def pins [cargo: record]: nothing -> record {
    let tables = ($KINDS | each {|k| {prefix: $k, deps: ($cargo | get -o $k | default {})} })
        | append ($cargo | get -o target | default {} | transpose cfg t | each {|row|
            $KINDS | each {|k| {prefix: $"($row.cfg) ($k)", deps: ($row.t | get -o $k | default {})} }
        } | flatten)
    $tables | each {|t|
        $t.deps | transpose name spec | each {|d|
            let v = if ($d.spec | describe) == "string" { $d.spec } else { $d.spec | get -o version | default "" }
            {key: $"($t.prefix) ($d.name)", value: $v}
        }
    } | flatten | reduce --fold {} {|it, acc| $acc | insert $it.key $it.value }
}

# Did a dependency requirement change between two parsed Cargo.toml files?
export def pins_changed [before: record, after: record]: nothing -> bool {
    (pins $before) != (pins $after)
}

# The next patch version: 0.4.7 → 0.4.8 (a pre-release ships as its release: 1.0.0-rc.1 → 1.0.0).
export def next_patch [version: string]: nothing -> string {
    let core = ($version | split row "-" | first | split row "." | each { into int })
    if ($version | str contains "-") { $core | str join "." } else { [$core.0 $core.1 ($core.2 + 1)] | str join "." }
}

export def upgrade_message [date: string] {
    $"chore\(deps\): upgrade dependencies ($date)"
}

def today [] { date now | date to-timezone UTC | format date "%Y-%m-%d" }

# The committed Cargo.toml, parsed.
def committed [] { ^git show HEAD:Cargo.toml | from toml }

def "main sync" [] {
    ^git fetch origin main
    ^git checkout -B main origin/main
    ^git reset --hard origin/main
    ^git log -1 --oneline
}

def "main update" [] {
    ^cargo update
    if ("fuzz/Cargo.toml" | path exists) { do { cd fuzz; ^cargo update } }
}

def "main changed" [] {
    let files = present $LOCKFILES
    let r = do { ^git diff --quiet -- ...$files } | complete
    let release = pins_changed (committed) (open Cargo.toml)
    let changed = $release or $r.exit_code != 0
    # stdout is $GITHUB_OUTPUT: key=value lines only; the human part goes to stderr.
    if $changed { print -e (^git diff --stat -- Cargo.toml ...$files) }
    print $"changed=($changed)"
    print $"release=($release)"
}

def "main commit" [bot: string, email: string] {
    ^git config user.name $bot
    ^git config user.email $email
    ^git add ...(present $LOCKFILES (ignored $LOCKFILES))
    ^git commit -m (commit_message (date now | date to-timezone UTC | format date "%Y-%m-%d"))
    ^git push origin HEAD:main
}

# Raise the compatible requirements in Cargo.toml (cargo-edit), then the lockfiles.
def "main upgrade" [] {
    ^cargo upgrade
    main update
}

# Commit the upgraded requirements, bump the patch version (changelog, tag) and push.
# The tag push starts release.yml only when it was made with a PAT (GH_PAT, so
# HAS_PAT=true); a GITHUB_TOKEN push starts no workflow, so then it is dispatched.
def "main release" [bot: string, email: string] {
    ^git config user.name $bot
    ^git config user.email $email
    ^git add Cargo.toml ...(present $LOCKFILES (ignored $LOCKFILES))
    ^git commit -m (upgrade_message (today))
    let next = next_patch (open Cargo.toml | get package.version)
    ^nu scripts/bump_version.nu $next --yes --skip-checks
    ^git push --follow-tags origin HEAD:main
    if ($env.HAS_PAT? | default "false") != "true" {
        ^gh workflow run release.yml --ref main -f $"tag=v($next)"
    }
    print $"released=v($next)"
}

def main [] { print "usage: nu scripts/ci/deps_update.nu (sync | update | upgrade | changed | commit <bot> <email> | release <bot> <email>)" }
