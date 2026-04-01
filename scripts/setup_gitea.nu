#!/usr/bin/env nu
# Set up Gitea as a second remote for dual-hosting (GitHub + Gitea).
# Usage: nu scripts/setup_gitea.nu <gitea-url>
#
# Examples (SSH recommended):
#   nu scripts/setup_gitea.nu git@gitea.example.com:user/tui-piechart.git
#   nu scripts/setup_gitea.nu https://gitea.example.com/user/tui-piechart.git

def main [
    gitea_url: string,  # Full Gitea remote URL (SSH or HTTPS)
] {
    let green  = (ansi green)
    let red    = (ansi red)
    let yellow = (ansi yellow)
    let blue   = (ansi blue)
    let cyan   = (ansi cyan)
    let reset  = (ansi reset)

    def success [msg: string] { print $"($green)✅ ($msg)($reset)" }
    def info    [msg: string] { print $"($blue)ℹ️  ($msg)($reset)" }
    def warning [msg: string] { print $"($yellow)⚠️  ($msg)($reset)" }
    def fail    [msg: string] { print $"($red)❌ ($msg)($reset)"; exit 1 }

    # ── Preflight checks ──────────────────────────────────────────────────────
    if (which git | length) == 0 {
        fail "git is not installed. Please install git first."
    }

    let git_check = (do { run-external "git" "rev-parse" "--git-dir" } | complete)
    if $git_check.exit_code != 0 {
        fail "Not a git repository. Please run this script from the tui-piechart directory."
    }

    print ""
    print $"($cyan)════════════════════════════════════════($reset)"
    print $"($cyan)  tui-piechart — Gitea Remote Setup($reset)"
    print $"($cyan)════════════════════════════════════════($reset)"
    print ""

    info $"Configuring Gitea remote: ($gitea_url)"
    print ""

    # ── Add or update the gitea remote ───────────────────────────────────────
    let existing_remotes = (do { run-external "git" "remote" } | complete)
    let has_gitea = ($existing_remotes.stdout | lines | any { |r| $r == "gitea" })

    if $has_gitea {
        warning "Gitea remote already exists — updating URL..."
        run-external "git" "remote" "set-url" "gitea" $gitea_url
        success "Gitea remote URL updated"
    } else {
        info "Adding Gitea remote..."
        run-external "git" "remote" "add" "gitea" $gitea_url
        success "Gitea remote added"
    }

    # ── Show current remotes ──────────────────────────────────────────────────
    print ""
    info "Current remotes:"
    let remotes = (do { run-external "git" "remote" "-v" } | complete)
    $remotes.stdout
    | lines
    | where { |l| $l =~ '^(origin|gitea)' }
    | each { |l| print $"  ($l)" }
    print ""

    # ── Test Gitea connection ─────────────────────────────────────────────────
    info "Testing Gitea repository connection..."
    let conn = (do { run-external "git" "ls-remote" "gitea" } | complete)
    if $conn.exit_code == 0 {
        success "Successfully connected to Gitea repository!"
    } else {
        warning "Could not connect to Gitea repository."
        print ""
        info "This is normal if the repository does not exist yet."
        print ""
        print "  To create the repository on Gitea:"
        print "    1. Log in to your Gitea instance"
        print "    2. Click '+' → New Repository"
        print "    3. Name it:  tui-piechart"
        print "    4. Do NOT initialise with a README"
        print "    5. Click 'Create Repository'"
        print "    6. Then run:  just push-gitea"
        print ""
    }

    # ── Optional push ─────────────────────────────────────────────────────────
    let reply = (input "Push all branches and tags to Gitea now? (y/N) ")
    print ""

    if ($reply =~ '^[Yy]') {
        info "Pushing all branches to Gitea..."
        let push_all = (do { run-external "git" "push" "gitea" "--all" } | complete)
        if $push_all.exit_code == 0 {
            success "All branches pushed to Gitea"
        } else {
            warning "Failed to push branches (repository might not exist yet)"
        }

        info "Pushing all tags to Gitea..."
        let push_tags = (do { run-external "git" "push" "gitea" "--tags" } | complete)
        if $push_tags.exit_code == 0 {
            success "All tags pushed to Gitea"
        } else {
            warning "Failed to push tags"
        }
    } else {
        info "Skipping push — run 'just push-gitea' when ready."
    }

    # ── Summary ───────────────────────────────────────────────────────────────
    print ""
    print $"($cyan)════════════════════════════════════════($reset)"
    print $"($green)✓ Gitea remote configured!($reset)"
    print $"($cyan)════════════════════════════════════════($reset)"
    print ""
    print "Quick commands:"
    print $"  ($cyan)just push-gitea($reset)       # Push branch to Gitea"
    print $"  ($cyan)just push-all($reset)         # Push to both GitHub and Gitea"
    print $"  ($cyan)just push-tags-all($reset)    # Push tags to both remotes"
    print $"  ($cyan)just release-all <ver>($reset) # Full release to both remotes"
    print $"  ($cyan)just remotes($reset)          # Show all configured remotes"
    print ""
}
