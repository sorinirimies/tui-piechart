#!/usr/bin/env nu
# Migrate a project to dual GitHub + Gitea hosting.
# Adds a Gitea remote, optionally pushes all history, and sets up .gitea/workflows.
#
# Usage: nu scripts/migrate_to_gitea.nu [--project-dir <dir>] [--gitea-url <url>]
# Examples:
#   nu scripts/migrate_to_gitea.nu
#   nu scripts/migrate_to_gitea.nu --gitea-url git@gitea.example.com:user/tui-piechart.git
#   nu scripts/migrate_to_gitea.nu --project-dir /path/to/project --gitea-url <url>

def main [
    --project-dir: string = "",  # Path to the project root (default: current directory)
    --gitea-url: string   = "",  # Full Gitea remote URL (prompted if not supplied)
] {
    let green  = (ansi green)
    let red    = (ansi red)
    let yellow = (ansi yellow)
    let blue   = (ansi blue)
    let cyan   = (ansi cyan)
    let bold   = (ansi --escape {attr: b})
    let reset  = (ansi reset)

    def success [msg: string] { print $"($green)✅ ($msg)($reset)" }
    def info    [msg: string] { print $"($blue)ℹ️  ($msg)($reset)" }
    def warning [msg: string] { print $"($yellow)⚠️  ($msg)($reset)" }
    def heading [msg: string] {
        print $"($cyan)($bold)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
        print $"($cyan)($bold)  ($msg)($reset)"
        print $"($cyan)($bold)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    }
    def fail [msg: string] {
        print $"($red)❌ Error: ($msg)($reset)"
        exit 1
    }

    # ── Resolve project directory ─────────────────────────────────────────────
    let proj_dir = if ($project_dir | is-empty) {
        $env.PWD
    } else {
        $project_dir | path expand
    }

    if not ($proj_dir | path exists) {
        fail $"Project directory does not exist: ($proj_dir)"
    }

    cd $proj_dir

    # ── Preflight checks ──────────────────────────────────────────────────────
    if (which git | length) == 0 {
        fail "git is not installed. Please install git first."
    }

    let git_check = (do { run-external "git" "rev-parse" "--git-dir" } | complete)
    if $git_check.exit_code != 0 {
        fail $"Not a git repository: ($proj_dir)"
    }

    let project_name = ($proj_dir | path basename)

    print ""
    heading $"Gitea Migration — ($project_name)"
    print ""
    info $"Project directory: ($proj_dir)"
    print ""

    # ── Resolve Gitea URL ─────────────────────────────────────────────────────
    let resolved_url = if ($gitea_url | is-empty) {
        print "Enter your Gitea remote URL."
        print "  SSH  example: git@gitea.example.com:user/tui-piechart.git"
        print "  HTTPS example: https://gitea.example.com/user/tui-piechart.git"
        print ""
        let url = (input "Gitea URL: " | str trim)
        if ($url | is-empty) {
            fail "No Gitea URL provided. Aborting."
        }
        $url
    } else {
        $gitea_url
    }

    # ── Add or update the gitea remote ───────────────────────────────────────
    heading "Configuring Gitea Remote"
    print ""

    let existing_remotes = (do { run-external "git" "remote" } | complete)
    let has_gitea = ($existing_remotes.stdout | lines | any { |r| ($r | str trim) == "gitea" })

    if $has_gitea {
        warning "Gitea remote already exists — updating URL..."
        run-external "git" "remote" "set-url" "gitea" $resolved_url
        success "Gitea remote URL updated"
    } else {
        info "Adding Gitea remote..."
        run-external "git" "remote" "add" "gitea" $resolved_url
        success "Gitea remote added"
    }

    # Show all remotes
    print ""
    info "Current remotes:"
    let remotes_out = (do { run-external "git" "remote" "-v" } | complete)
    $remotes_out.stdout
    | lines
    | where { |l| $l =~ '^(origin|gitea)' }
    | each { |l| print $"  ($l)" }
    print ""

    # ── Test Gitea connection ─────────────────────────────────────────────────
    heading "Testing Gitea Connection"
    print ""

    info "Testing connection to Gitea repository..."
    let conn = (do { run-external "git" "ls-remote" "gitea" } | complete)
    if $conn.exit_code == 0 {
        success "Successfully connected to Gitea repository!"
    } else {
        warning "Could not connect to Gitea repository."
        print ""
        info "This is normal if the repository does not exist yet."
        print ""
        print "  To create the repository on Gitea:"
        print $"    1. Log in to your Gitea instance"
        print $"    2. Create repository: ($project_name)"
        print "    3. Do NOT initialise with a README"
        print "    4. Run:  just push-gitea"
        print ""
    }

    # ── Push to Gitea ─────────────────────────────────────────────────────────
    heading "Push to Gitea"
    print ""

    let push_reply = (input "Push all branches and tags to Gitea now? (y/N) " | str trim)
    print ""

    if ($push_reply =~ '^[Yy]') {
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

    # ── Set up .gitea/workflows ───────────────────────────────────────────────
    print ""
    heading "Gitea Actions (CI/CD)"
    print ""

    let gitea_reply = (input "Set up .gitea/workflows from .github/workflows? (y/N) " | str trim)
    print ""

    if ($gitea_reply =~ '^[Yy]') {
        let gitea_dir = ($proj_dir | path join ".gitea" "workflows")

        if ($gitea_dir | path exists) {
            success ".gitea/workflows already exists — skipping copy"
        } else {
            let github_dir = ($proj_dir | path join ".github" "workflows")
            if ($github_dir | path exists) {
                info "Copying workflows from .github/workflows → .gitea/workflows..."
                mkdir $gitea_dir
                let workflow_files = (glob ($github_dir | path join "*.yml"))
                for wf in $workflow_files {
                    let dest = ($gitea_dir | path join ($wf | path basename))
                    cp $wf $dest
                }
                success "Workflows copied to .gitea/workflows/"
                info "Review and adjust them for Gitea Actions syntax if needed."
            } else {
                mkdir $gitea_dir
                success ".gitea/workflows directory created"
                info "Add your workflow YAML files to .gitea/workflows/"
            }
        }
    } else {
        info "Skipping .gitea setup."
    }

    # ── Summary ───────────────────────────────────────────────────────────────
    print ""
    heading "Migration Complete! 🎉"
    print ""
    success $"Project ($project_name) configured for dual GitHub + Gitea hosting!"
    print ""
    info "What was done:"
    print $"  ✓ Gitea remote set to: ($resolved_url)"
    print ""
    info "Quick commands:"
    print $"  ($cyan)just push($reset)              # Push to GitHub only"
    print $"  ($cyan)just push-gitea($reset)        # Push to Gitea only"
    print $"  ($cyan)just push-all($reset)          # Push to both GitHub and Gitea"
    print $"  ($cyan)just push-tags-all($reset)     # Push tags to both remotes"
    print $"  ($cyan)just sync-gitea($reset)        # Force-sync Gitea from GitHub"
    print $"  ($cyan)just remotes($reset)           # Show all configured remotes"
    print ""
    info "Release commands:"
    print $"  ($cyan)just release 0.3.2($reset)         # Release to GitHub"
    print $"  ($cyan)just release-gitea 0.3.2($reset)   # Release to Gitea"
    print $"  ($cyan)just release-all 0.3.2($reset)     # Release to both remotes"
    print ""
    success "Happy dual-hosting! 🚀"
    print ""
}
