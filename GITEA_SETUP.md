# Quick Gitea Setup Guide

This guide helps you set up dual hosting for tui-slider on GitHub and your Gitea instance.

## Prerequisites

- ✅ Existing GitHub repository (already set up)
- ✅ Access to your Gitea instance
- ✅ **SSH keys configured** (required for password-free workflow)

## SSH Key Setup (Required First)

Before setting up Gitea, you need SSH keys configured for password-free access.

### Check if You Have SSH Keys

```bash
# Check for existing SSH keys
ls -la ~/.ssh/

# You should see id_ed25519 or id_rsa files
```

### Generate SSH Keys (if needed)

```bash
# Generate a new SSH key (Ed25519 is recommended)
ssh-keygen -t ed25519 -C "your_email@example.com"

# Or RSA if Ed25519 is not available
ssh-keygen -t rsa -b 4096 -C "your_email@example.com"

# Press Enter to accept default location
# Optionally set a passphrase (or press Enter for none)
```

### Add SSH Key to Gitea

```bash
# Copy your public key
cat ~/.ssh/id_ed25519.pub
# Or for RSA: cat ~/.ssh/id_rsa.pub

# Then add to Gitea:
# 1. Log in to your Gitea instance
# 2. Click your avatar → Settings
# 3. Go to "SSH / GPG Keys" tab
# 4. Click "Add Key"
# 5. Paste your public key
# 6. Give it a name (e.g., "work-laptop")
# 7. Click "Add Key"
```

### Test SSH Connection

```bash
# Test connection to your Gitea instance
ssh -T git@gitea.yourdomain.com

# You should see a success message like:
# "Hi there, username! You've successfully authenticated..."
```

## Quick Setup

### Option 1: Automated Setup (Recommended)

Run the setup script with SSH URL:

```bash
# Always use SSH format (git@hostname:path)
./scripts/setup-gitea.sh git@gitea.yourdomain.com:username/tui-slider.git
```

The script will:
- Add Gitea as a remote
- Test the connection
- Optionally push all branches and tags
- Set up Gitea Actions (CI/CD)

### Option 2: Manual Setup

```bash
# 1. Add Gitea remote (SSH format)
git remote add gitea git@gitea.yourdomain.com:username/tui-slider.git

# 2. Test connection
ssh -T git@gitea.yourdomain.com

# 3. Push all branches
git push gitea --all

# 4. Push all tags
git push gitea --tags

# 5. Verify
git remote -v
```

## Daily Usage

### Push to Both Remotes

```bash
# Using justfile (recommended)
just push-all

# Or manually
git push origin main
git push gitea main
```

### Create a Release

```bash
# Automatically pushes to both GitHub and Gitea
just release 0.2.0
```

### Check Remote Status

```bash
just remotes
```

## Available Commands

```bash
just push           # Push to GitHub only
just push-gitea     # Push to Gitea only
just push-all       # Push to both remotes
just push-tags-all  # Push tags to both remotes
just sync-gitea     # Force sync Gitea with GitHub
just remotes        # Show configured remotes
```

## Gitea Configuration

### 1. Create Repository on Gitea

1. Log in to your Gitea instance
2. Click "+" → "New Repository"
3. Repository name: `tui-slider`
4. Description: "A simple TUI slider component library for ratatui"
5. Visibility: Public or Private (your choice)
6. **Do NOT** initialize with README (we'll push from local)
7. Click "Create Repository"

### 2. Verify SSH Keys

SSH keys should already be configured (see Prerequisites above).

```bash
# Verify SSH connection
ssh -T git@gitea.yourdomain.com

# Should show: "Hi there, username! You've successfully authenticated..."
```

If not working, check:
```bash
# Check SSH keys exist
ls -la ~/.ssh/

# View public key (add this to Gitea)
cat ~/.ssh/id_ed25519.pub  # or id_rsa.pub

# Test with verbose output
ssh -Tv git@gitea.yourdomain.com
```

### 3. Configure Gitea Actions (Optional)

If your Gitea instance supports Actions (Gitea 1.19+):

```bash
# Create Gitea workflows directory
mkdir -p .gitea/workflows

# Copy GitHub workflows
cp .github/workflows/ci.yml .gitea/workflows/
cp .github/workflows/release.yml .gitea/workflows/

# Commit
git add .gitea/
git commit -m "ci: add Gitea Actions workflows"
git push gitea main
```

## Syncing Strategies

### Strategy 1: Always Push to Both (Recommended)

```bash
# Set up in .git/config
git remote set-url --add --push origin git@gitea.yourdomain.com:username/tui-slider.git
```

Now `git push origin main` pushes to both!

### Strategy 2: Use Justfile Commands

```bash
# Already configured in justfile
just push-all      # Pushes to both
just release 0.2.0 # Releases to both
```

### Strategy 3: GitHub Primary, Gitea Mirror

```bash
# Push to GitHub first
git push origin main

# Then sync to Gitea
just sync-gitea
```

## CI/CD on Gitea

### Gitea Actions (GitHub Actions Compatible)

If using Gitea Actions, workflows in `.gitea/workflows/` run automatically.

Configure runners:
1. Gitea Admin Panel → Actions → Runners
2. Follow runner setup instructions
3. Workflows will execute on push/PR

### Drone CI

If using Drone CI, create `.drone.yml`:

```yaml
kind: pipeline
type: docker
name: default

steps:
  - name: test
    image: rust:latest
    commands:
      - cargo fmt -- --check
      - cargo clippy -- -D warnings
      - cargo test --all-features
```

### Woodpecker CI

If using Woodpecker, create `.woodpecker.yml`:

```yaml
pipeline:
  test:
    image: rust:latest
    commands:
      - cargo test --all-features
```

## Troubleshooting

### Permission Denied (SSH)

```bash
# Test SSH connection
ssh -T git@gitea.yourdomain.com

# If fails, add SSH key to Gitea
cat ~/.ssh/id_ed25519.pub

# Add to Gitea:
# 1. Log in to Gitea
# 2. Settings → SSH/GPG Keys → Add Key
# 3. Paste the public key content
# 4. Save

# Test again
ssh -T git@gitea.yourdomain.com
```

### Switch from HTTPS to SSH

HTTPS requires passwords. Use SSH instead:

```bash
# Use token authentication
git remote set-url gitea https://username:token@gitea.yourdomain.com/username/tui-slider.git

# Generate token in Gitea:
# Settings → Applications → Generate New Token
```

### Push Failed - Repository Not Found

```bash
# Make sure repository exists on Gitea
# Create it via Gitea UI first, then:
git push gitea --all --force
```

### Remotes Out of Sync

```bash
# Fetch from both
git fetch --all

# Force sync Gitea with GitHub
just sync-gitea
```

## Verification

Check your setup:

```bash
# 1. View remotes
just remotes

# Should show:
# origin  git@github.com:sorinirimies/tui-slider.git
# gitea   git@gitea.yourdomain.com:username/tui-slider.git

# 2. Test push to Gitea
git push gitea main --dry-run

# 3. Check branches on both remotes
git branch -r
```

## Migration Back to Single Remote

If you want to remove Gitea hosting:

```bash
# Remove Gitea remote
git remote remove gitea

# Update justfile commands (revert to single remote)
# Edit justfile: remove gitea-related commands
```

## Resources

- 📖 [Full Dual Hosting Guide](./DUAL_HOSTING.md)
- 🔧 [Justfile Commands](./justfile) - Run `just --list`
- 🚀 [Release Workflow](./RELEASE.md)
- 🌐 [Gitea Documentation](https://docs.gitea.io/)

## Quick Reference

```bash
# Prerequisites (first time only)
ssh-keygen -t ed25519 -C "your_email@example.com"  # Generate key
cat ~/.ssh/id_ed25519.pub                           # Copy public key
# Add to Gitea: Settings → SSH/GPG Keys → Add Key
ssh -T git@gitea.example.com                        # Test connection

# Setup
./scripts/setup-gitea.sh git@gitea.example.com:user/tui-slider.git

# Daily use
just push-all                # Push to both
just release 0.2.0           # Release to both

# Maintenance
just sync-gitea              # Sync Gitea with GitHub
just remotes                 # Show remotes

# Manual
git push origin main         # GitHub only
git push gitea main          # Gitea only
```

## Success Criteria

You've successfully set up Gitea when:

- ✅ SSH key added to Gitea (Settings → SSH Keys)
- ✅ `ssh -T git@gitea.example.com` shows success message
- ✅ `just remotes` shows both origin and gitea
- ✅ `git push gitea main` works **without password prompt**
- ✅ `just push-all` pushes to both remotes
- ✅ Repository is visible on your Gitea instance
- ✅ `just release X.Y.Z` pushes to both GitHub and Gitea

Need help? Check [DUAL_HOSTING.md](./DUAL_HOSTING.md) for detailed information.