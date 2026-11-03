#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Create a Gitea release and upload the archives
# ──────────────────────────────────────────────────────────────────────────────
# Usage (in .gitea/workflows/release.yml):
#   nu scripts/ci/gitea_release.nu <tag> --api <server>/api/v1/repos/<owner>/<repo> --dir dist
# Env: GITEA_TOKEN (the workflow passes secrets.GITHUB_TOKEN). Notes: RELEASE_NOTES.md.
# ──────────────────────────────────────────────────────────────────────────────

export def release_payload [tag: string, title: string, notes: string] {
    { tag_name: $tag, name: $"($title) ($tag)", body: $notes }
}

def main [tag: string, --api: string, --dir: path = "dist", --title: string = "tui-piechart"] {
    let headers = { Authorization: $"token ($env.GITEA_TOKEN)" }
    let notes = if ("RELEASE_NOTES.md" | path exists) { open --raw RELEASE_NOTES.md } else { "" }
    let release = http post --content-type application/json --headers $headers $"($api)/releases" (release_payload $tag $title $notes)
    for f in (glob --no-dir $"($dir)/*") {
        let name = $f | path basename
        http post --content-type application/octet-stream --headers $headers $"($api)/releases/($release.id)/assets?name=($name)" (open --raw $f) | ignore
        print $"📦 ($name)"
    }
}
