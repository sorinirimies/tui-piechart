# Dual Hosting Strategy: GitHub + Gitea

This guide explains how to host tui-slider on both GitHub and your own Gitea instance.

## Overview

**Strategy:** Dual Primary with Automatic Sync

- **GitHub** - Primary public repository (for discoverability, crates.io, community)
- **Gitea** - Secondary/backup repository (for self-hosting, control, redundancy)

Both repositories are kept in sync automatically.

## Benefits

- ✅ **Redundancy** - Multiple hosting locations
- ✅ **Control** - Own your code on your infrastructure
- ✅ **Visibility** - Public on GitHub for discoverability
- ✅ **Flexibility** - Can switch primary at any time
- ✅ **CI/CD** - Run workflows on both platforms
- ✅ **Backup** - Automatic offsite backup

## Setup

### 1. Add Gitea Remote

Add your Gitea instance as a second remote:

```bash
# Add Gitea remote
git remote add gitea https://gitea.yourdomain.com/username/tui-slider.git

# Or with SSH (recommended)
git remote add gitea git@gitea.yourdomain.com:username/tui-slider.git

# Verify remotes
git remote -v
```

You should see:

```
origin  git@github.com:sorinirimies/tui-slider.git (fetch)
origin  git@github.com:sorinirimies/tui-slider.git (push)
gitea   git@gitea.yourdomain.com:username/tui-slider.git (fetch)
gitea   git@gitea.yourdomain.com:username/tui-slider.git (push)
```

### 2. Initial Push to Gitea

```bash
# Push all branches and tags to Gitea
git push gitea --all
git push gitea --tags
```

### 3. Configure Push to Both Remotes

**Option A: Push to Both with One Command**

Add Gitea as an additional push URL for origin:

```bash
git remote set-url --add --push origin git@gitea.yourdomain.com:username/tui-slider.git
```

Now `git push origin` will push to both GitHub and Gitea!

**Option B: Create an "all" Remote**

```bash
git remote add all git@github.com:sorinirimies/tui-slider.git
git remote set-url --add --push all git@github.com:sorinirimies/tui-slider.git
git remote set-url --add --push all git@gitea.yourdomain.com:username/tui-slider.git

# Push to both
git push all
git push all --tags
```

### 4. Update Justfile Commands

Add Gitea-aware commands to your justfile:

```just
# Push to GitHub only
push:
    git push origin main

# Push to Gitea only
push-gitea:
    git push gitea main

# Push to both GitHub and Gitea
push-all:
    git push origin main
    git push gitea main
    @echo "✅ Pushed to both GitHub and Gitea!"

# Push tags to both
push-tags-all:
    git push origin --tags
    git push gitea --tags
    @echo "✅ Tags pushed to both remotes!"

# Complete release to both platforms
push-release-all:
    git push origin main
    git push gitea main
    git push origin --tags
    git push gitea --tags
    @echo "✅ Release pushed to both GitHub and Gitea!"

# Show remote status
remotes:
    @echo "Configured remotes:"
    @git remote -v
```

Then update the `release` command to use `push-release-all`:

```just
# Bump version (usage: just bump 0.2.0)
bump version: check-git-cliff
    @echo "Bumping version to {{version}}..."
    @# Update version in Cargo.toml
    @sed -i.bak 's/^version = ".*"/version = "{{version}}"/' Cargo.toml && rm Cargo.toml.bak
    @# Update Cargo.lock
    @cargo build
    @# Generate changelog for this version
    @git-cliff --tag v{{version}} -o CHANGELOG.md
    @# Commit changes
    @git add Cargo.toml Cargo.lock CHANGELOG.md
    @git commit -m "chore(release): bump version to {{version}}"
    @# Create git tag
    @git tag -a v{{version}} -m "Release v{{version}}"
    @echo "✅ Version bumped to {{version}}!"
    @echo "📝 Changelog updated"
    @echo "🏷️  Tag v{{version}} created"
    @echo ""
    @echo "Pushing to both GitHub and Gitea..."
    @git push origin main
    @git push gitea main
    @git push origin v{{version}}
    @git push gitea v{{version}}
    @echo "✅ Release v{{version}} pushed to both remotes!"
```

## CI/CD for Gitea

### Gitea Actions (GitHub Actions Compatible)

If your Gitea instance has Gitea Actions enabled (Gitea 1.19+), you can use the same workflows!

Create `.gitea/workflows/` (note: `.gitea` not `.github`):

```bash
mkdir -p .gitea/workflows
cp .github/workflows/ci.yml .gitea/workflows/ci.yml
cp .github/workflows/release.yml .gitea/workflows/release.yml
```

**Modify for Gitea:**

1. Update secrets names if needed
2. Adjust any GitHub-specific actions
3. Configure Gitea Actions runners

### Drone CI (Alternative)

If using Drone CI with Gitea:

Create `.drone.yml`:

```yaml
kind: pipeline
type: docker
name: default

steps:
  - name: format
    image: rust:latest
    commands:
      - cargo fmt -- --check

  - name: clippy
    image: rust:latest
    commands:
      - cargo clippy -- -D warnings

  - name: test
    image: rust:latest
    commands:
      - cargo test --all-features

  - name: build
    image: rust:latest
    commands:
      - cargo build --release

trigger:
  branch:
    - main
    - develop
  event:
    - push
    - pull_request
```

### Woodpecker CI (Alternative)

If using Woodpecker CI:

Create `.woodpecker.yml`:

```yaml
pipeline:
  format:
    image: rust:latest
    commands:
      - cargo fmt -- --check

  clippy:
    image: rust:latest
    commands:
      - cargo clippy -- -D warnings

  test:
    image: rust:latest
    commands:
      - cargo test --all-features

  build:
    image: rust:latest
    commands:
      - cargo build --release
```

## Daily Workflow

### Regular Development

```bash
# Make changes
git add .
git commit -m "feat: add new feature"

# Push to both remotes
just push-all

# Or use git directly
git push origin main
git push gitea main
```

### Creating a Release

```bash
# Use the updated justfile command
just release 0.2.0

# This will automatically:
# 1. Run all checks
# 2. Bump version
# 3. Generate changelog
# 4. Create commit and tag
# 5. Push to BOTH GitHub and Gitea
```

### Pull from Either Remote

```bash
# Pull from GitHub (default)
git pull origin main

# Pull from Gitea
git pull gitea main

# Fetch from both
git fetch --all
```

## Sync Strategies

### Strategy 1: GitHub as Primary (Recommended)

**Use Case:** Public project, want GitHub visibility for crates.io

```bash
# Always push to GitHub first
git push origin main

# Then sync to Gitea
git push gitea main

# Or use the all remote
git push all main
```

**Benefits:**
- GitHub is the source of truth
- Gitea serves as backup/mirror
- Crates.io releases work seamlessly

### Strategy 2: Gitea as Primary

**Use Case:** Private development, push to GitHub for releases only

```bash
# Daily work pushes to Gitea
git push gitea main

# When ready to release, push to GitHub
git push origin main
git push origin --tags
```

**Benefits:**
- Keep development private on your server
- Control when code goes public
- Use Gitea for team collaboration

### Strategy 3: Equal Primary (Current Setup)

**Use Case:** Maximum redundancy and flexibility

```bash
# Always push to both
just push-all

# For releases
just release 0.2.0  # Updated to push to both
```

**Benefits:**
- Full redundancy
- Either can be primary if needed
- No single point of failure

## Automatic Mirroring

### Option A: Git Hook (Local)

Create `.git/hooks/post-push`:

```bash
#!/bin/bash
# Automatically push to Gitea after pushing to GitHub

REMOTE_GITEA="gitea"

# Get current branch
BRANCH=$(git rev-parse --abbrev-ref HEAD)

echo "Syncing to Gitea..."
git push $REMOTE_GITEA $BRANCH

# Also push tags
git push $REMOTE_GITEA --tags 2>/dev/null

echo "✅ Synced to Gitea!"
```

Make it executable:

```bash
chmod +x .git/hooks/post-push
```

### Option B: Gitea Repository Mirroring

In Gitea UI:
1. Go to Repository Settings
2. Enable "Mirror Repository"
3. Set GitHub as source
4. Configure sync interval

This pulls from GitHub automatically at intervals.

### Option C: GitHub Action to Push to Gitea

Create `.github/workflows/sync-gitea.yml`:

```yaml
name: Sync to Gitea

on:
  push:
    branches: [main, develop]
  release:
    types: [published]

jobs:
  sync:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Push to Gitea
        env:
          GITEA_URL: ${{ secrets.GITEA_URL }}
          GITEA_TOKEN: ${{ secrets.GITEA_TOKEN }}
        run: |
          git remote add gitea $GITEA_URL
          git push gitea main --force
          git push gitea --tags --force
```

Add secrets to GitHub:
- `GITEA_URL`: Your Gitea repository URL
- `GITEA_TOKEN`: Gitea access token

## README Badges

Update your README.md to show both hosting locations:

```markdown
[![GitHub](https://img.shields.io/badge/GitHub-sorinirimies%2Ftui--slider-blue?logo=github)](https://github.com/sorinirimies/tui-slider)
[![Gitea](https://img.shields.io/badge/Gitea-self--hosted-green?logo=gitea)](https://gitea.yourdomain.com/username/tui-slider)

Hosted on:
- 🐙 [GitHub](https://github.com/sorinirimies/tui-slider) - Primary public repository
- 🍵 [Gitea](https://gitea.yourdomain.com/username/tui-slider) - Self-hosted mirror
```

## Troubleshooting

### Push Fails to Gitea

```bash
# Check remote configuration
git remote -v

# Test connection
ssh -T git@gitea.yourdomain.com

# Force push if needed (be careful!)
git push gitea main --force
```

### Remotes Out of Sync

```bash
# Fetch from both
git fetch --all

# Compare branches
git log origin/main..gitea/main

# Sync Gitea with GitHub
git push gitea origin/main:main --force
```

### Authentication Issues

**For SSH:**
```bash
# Add SSH key to Gitea
cat ~/.ssh/id_rsa.pub

# Add to Gitea UI: Settings → SSH/GPG Keys
```

**For HTTPS:**
```bash
# Store credentials
git config credential.helper store

# Or use token
git remote set-url gitea https://username:token@gitea.yourdomain.com/username/tui-slider.git
```

## Best Practices

1. **Always push to both remotes** - Use justfile commands or git hooks
2. **Keep remotes in sync** - Don't let them diverge
3. **Use GitHub for releases** - For crates.io integration and visibility
4. **Use Gitea for backups** - Automatic redundancy
5. **Document URLs** - Update README with both repository links
6. **Test both CIs** - Ensure workflows work on both platforms
7. **Tag consistently** - Push tags to both remotes

## Security Considerations

1. **Gitea Access Control**
   - Set appropriate permissions
   - Use SSH keys for authentication
   - Enable 2FA if available

2. **Secrets Management**
   - Keep `CRATES_IO_TOKEN` only on GitHub
   - Use separate tokens for each platform
   - Never commit credentials

3. **Private vs Public**
   - GitHub: Usually public
   - Gitea: Can be private for development
   - Be careful what you push where

## Migration Plans

### If GitHub Goes Down

```bash
# Switch primary to Gitea
git remote rename origin github
git remote rename gitea origin

# Update URLs in documentation
# Continue development
```

### If Gitea Goes Down

```bash
# No action needed
# GitHub is still operational
# Restore Gitea when ready

# Re-sync after restoration
git push gitea --all
git push gitea --tags
```

## Summary Commands

```bash
# Initial setup
git remote add gitea git@gitea.yourdomain.com:username/tui-slider.git

# Push to both
just push-all

# Release to both
just release 0.2.0

# Check status
just remotes

# Sync manually
git push gitea main --force
git push gitea --tags --force
```

## Resources

- [Gitea Documentation](https://docs.gitea.io/)
- [Gitea Actions](https://docs.gitea.io/en-us/usage/actions/overview/)
- [Git Remote Documentation](https://git-scm.com/docs/git-remote)
- [Multiple Remote Workflows](https://git-scm.com/book/en/v2/Git-Basics-Working-with-Remotes)