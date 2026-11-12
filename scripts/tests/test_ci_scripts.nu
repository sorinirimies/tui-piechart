#!/usr/bin/env nu
# ── tui-piechart · test_ci_scripts.nu ───────────────────────────────────
# The decisions the CI scripts make, without network, git or cargo.

use std/assert
use runner.nu *
use ../ci/deps_update.nu [present commit_message]
use ../ci/deps_update.nu [pins pins_changed next_patch upgrade_message]
use ../ci/publish_crate.nu [already_published]
use ../ci/gitea_release.nu [release_payload]
use ../ci/auto_merge.nu [decide checks_green sweep_action]
use ../ci/actions_update.nu [uses major_of bump PINNED]

def "test deps_update: commit message is a dated chore deps" [] {
    assert equal (commit_message "2026-01-02") "chore(deps): nightly cargo update 2026-01-02"
}

def "test deps_update: only existing lockfiles are touched" [] {
    let root = ($env.CURRENT_FILE | path dirname | path join ".." "..")
    cd $root
    assert equal (present ["Cargo.lock" "no/such/Cargo.lock"]) ["Cargo.lock"]
    # An ignored lockfile (some repos ignore fuzz/Cargo.lock) is not staged.
    assert equal (present ["Cargo.toml" "scripts/project.nu"] ["scripts/project.nu"]) ["Cargo.toml"]
}

def "test deps_update: a raised requirement is a release, formatting is not" [] {
    let before = "[package]\nname = \"x\"\nversion = \"0.1.0\"\n\n[dependencies]\na = \"1.0\"\nb = { version = \"0.3\", optional = true }\n" | from toml
    let reordered = "[package]\nversion = \"0.1.0\"\nname = \"x\"\n[dependencies]\nb = { optional = true, version = \"0.3\" }\na = \"1.0\"\n" | from toml
    assert not (pins_changed $before $reordered)
    assert (pins_changed $before ($before | upsert dependencies.a "1.0.5"))
    assert (pins_changed $before ($before | upsert dependencies.b.version "0.3.2"))
}

def "test deps_update: dev and target requirements count" [] {
    let base = {package: {name: x}, dev-dependencies: {c: "2"}, target: {"cfg(unix)": {dependencies: {d: "0.1"}}}}
    assert equal (pins $base) {"dev-dependencies c": "2", "cfg(unix) dependencies d": "0.1"}
    assert (pins_changed $base ($base | upsert target."cfg(unix)".dependencies.d "0.2"))
    assert not (pins_changed $base $base)
}

def "test deps_update: the next patch version" [] {
    assert equal (next_patch "0.4.7") "0.4.8"
    assert equal (next_patch "1.2.9") "1.2.10"
    assert equal (next_patch "1.0.0-rc.1") "1.0.0"
    assert equal (upgrade_message "2026-01-02") "chore(deps): upgrade dependencies 2026-01-02"
}

def "test publish_crate: a re-release counts as success" [] {
    assert (already_published "error: crate version `0.1.0` is already uploaded")
    assert (already_published "error: crate kv@0.1.0 already exists on crates.io index")
    assert not (already_published "error: failed to verify package tarball")
}


const PR = {number: 7, author: {login: "app/dependabot"}, headRefOid: "abc", state: "OPEN"}

def "test auto_merge: a green dependabot pr is merged, majors included" [] {
    assert (decide ($PR | insert title "ci(deps): bump softprops/action-gh-release from 2 to 3") "abc").merge
}

def "test auto_merge: others, closed prs and moved branches are left" [] {
    assert not (decide ($PR | update author {login: "sorinirimies"}) "abc").merge
    assert not (decide ($PR | update state "MERGED") "abc").merge
    assert not (decide $PR "def").merge
}

def "test auto_merge: the sweep needs every check passed" [] {
    assert (checks_green [{conclusion: "SUCCESS"} {conclusion: "SKIPPED"} {state: "SUCCESS"}])
    assert not (checks_green [{conclusion: "SUCCESS"} {conclusion: "FAILURE"}])
    assert not (checks_green [{conclusion: "SUCCESS"} {status: "IN_PROGRESS", conclusion: ""}])
    assert not (checks_green [])
}

def "test auto_merge: the sweep merges green prs, rebases conflicts and waits otherwise" [] {
    let pr = $PR | insert mergeable "MERGEABLE" | insert statusCheckRollup [{conclusion: "SUCCESS"}]
    assert equal (sweep_action $pr) "merge"
    assert equal (sweep_action ($pr | update mergeable "CONFLICTING")) "rebase"
    assert equal (sweep_action ($pr | update statusCheckRollup [{conclusion: "FAILURE"}])) "wait"
    assert equal (sweep_action ($pr | update author {login: "someone"})) "wait"
}

def "test gitea_release: payload carries tag, title and notes" [] {
    assert equal (release_payload "v1.2.3" "KV" "notes") { tag_name: "v1.2.3", name: "KV v1.2.3", body: "notes" }
}

def "test actions_update: finds every action and major" [] {
    let text = "steps:\n  - uses: actions/checkout@v4\n  - uses: Swatinem/rust-cache@v2\n  - uses: hustcer/setup-nu@v3\n"
    assert equal (uses $text | get repo) ["actions/checkout" "Swatinem/rust-cache" "hustcer/setup-nu"]
    assert equal (uses $text | get major) [4 2 3]
}

def "test actions_update: major of a release tag" [] {
    assert equal (major_of "v5.2.0") 5
    assert equal (major_of "v12") 12
    assert equal (major_of "release-1") null
}

def "test actions_update: bumps older majors only" [] {
    let text = "- uses: Swatinem/rust-cache@v2\n- uses: hustcer/setup-nu@v3\n- uses: foo/bar@v9\n"
    let out = bump $text { "Swatinem/rust-cache": 3, "hustcer/setup-nu": 3, "foo/bar": 1 }
    assert ($out | str contains "Swatinem/rust-cache@v3")
    assert ($out | str contains "hustcer/setup-nu@v3")
    assert ($out | str contains "foo/bar@v9")
}

def "test actions_update: pinned actions never move" [] {
    let text = "- uses: actions/checkout@v4\n"
    assert equal (bump $text { "actions/checkout": 6 }) $text
    assert ("actions/upload-artifact" in $PINNED)
}

def "test actions_update: v1 does not touch v10" [] {
    let out = bump "- uses: a/b@v1\n- uses: a/b@v10\n" { "a/b": 2 }
    assert equal $out "- uses: a/b@v2\n- uses: a/b@v10\n"
}

def "test gitattributes: gifs are plain git objects, no lfs clean filter" [] {
    # Regression: `*.gif filter=lfs` + a later `*.gif binary` left the filter set, so on
    # a runner with git-lfs every committed GIF showed as modified and the nightly
    # release stopped at "the working tree is dirty".
    let root = ($env.CURRENT_FILE | path dirname | path join ".." "..")
    let out = (^git -C $root check-attr filter -- examples/vhs/output/piechart.gif | str trim)
    assert ($out | str ends-with "unset") $"gif filter must be unset, got: ($out)"
}

def main [] { run-tests }
