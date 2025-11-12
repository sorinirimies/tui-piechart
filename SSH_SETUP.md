# SSH Setup Guide for Gitea

Quick guide to set up SSH keys for password-free Git operations with your Gitea instance.

## Why SSH?

- ✅ **No passwords** - Authenticate with keys instead
- ✅ **More secure** - Cryptographic authentication
- ✅ **Better workflow** - No interruptions for credentials
- ✅ **Required for automation** - Scripts and CI/CD need it

## Quick Setup (3 Steps)

### 1. Generate SSH Key

```bash
# Generate Ed25519 key (recommended, modern and secure)
ssh-keygen -t ed25519 -C "your_email@example.com"

# Or RSA if Ed25519 is not supported
ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
```

**Prompts:**
- `Enter file in which to save the key`: Press Enter (use default)
- `Enter passphrase`: Press Enter for no passphrase (or set one for extra security)
- `Enter same passphrase again`: Press Enter again

**Output:**
```
Your identification has been saved in /home/user/.ssh/id_ed25519
Your public key has been saved in /home/user/.ssh/id_ed25519.pub
```

### 2. Copy Your Public Key

```bash
# Display your public key
cat ~/.ssh/id_ed25519.pub

# Or for RSA
cat ~/.ssh/id_rsa.pub
```

**Example output:**
```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILw... your_email@example.com
```

Copy the entire line (starts with `ssh-ed25519` or `ssh-rsa`).

### 3. Add Key to Gitea

1. **Log in** to your Gitea instance
2. Click your **avatar** (top right) → **Settings**
3. Click **SSH / GPG Keys** tab
4. Click **Add Key** button
5. **Title**: Give it a name (e.g., "Work Laptop", "Home PC")
6. **Content**: Paste your public key
7. Click **Add Key**

### 4. Test Connection

```bash
# Test SSH connection to your Gitea instance
ssh -T git@gitea.yourdomain.com

# Expected success message:
# Hi there, username! You've successfully authenticated...
```

✅ **Success!** You can now use Git with SSH without passwords.

## Detailed Setup

### Check Existing Keys

Before generating new keys, check if you already have them:

```bash
# List SSH keys
ls -la ~/.ssh/

# Look for these files:
# id_ed25519 and id_ed25519.pub  (Ed25519 key pair)
# id_rsa and id_rsa.pub          (RSA key pair)
```

If you see these files, you already have SSH keys! Skip to step 2 (Copy Your Public Key).

### Multiple SSH Keys

If you use multiple Git services (GitHub, GitLab, Gitea), you can:

**Option 1: Use the Same Key** (easiest)
- Add your public key to all services
- Works everywhere automatically

**Option 2: Use Different Keys** (more complex)
- Generate separate keys for each service
- Configure `~/.ssh/config` to use the right key

Example `~/.ssh/config` for multiple keys:

```
# GitHub
Host github.com
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_ed25519_github

# Gitea
Host gitea.yourdomain.com
    HostName gitea.yourdomain.com
    User git
    IdentityFile ~/.ssh/id_ed25519_gitea
```

### SSH Agent (Optional)

If you set a passphrase on your key, use ssh-agent to avoid typing it repeatedly:

```bash
# Start ssh-agent
eval "$(ssh-agent -s)"

# Add your key
ssh-add ~/.ssh/id_ed25519

# Verify
ssh-add -l
```

On macOS, add to `~/.ssh/config`:
```
Host *
    AddKeysToAgent yes
    UseKeychain yes
    IdentityFile ~/.ssh/id_ed25519
```

## Troubleshooting

### Problem: "Permission denied (publickey)"

**Causes:**
- SSH key not added to Gitea
- Wrong SSH key being used
- SSH agent not running

**Solutions:**

```bash
# 1. Check if key exists
ls -la ~/.ssh/id_ed25519.pub

# 2. View and copy your public key
cat ~/.ssh/id_ed25519.pub

# 3. Add to Gitea: Settings → SSH Keys → Add Key

# 4. Test connection with verbose output
ssh -Tv git@gitea.yourdomain.com

# Look for lines like:
# debug1: Offering public key: /home/user/.ssh/id_ed25519
# debug1: Server accepts key: /home/user/.ssh/id_ed25519
```

### Problem: "Could not resolve hostname"

```bash
# Check hostname is correct
ping gitea.yourdomain.com

# Verify remote URL
git remote get-url gitea

# Should be: git@gitea.yourdomain.com:username/repo.git
```

### Problem: "Host key verification failed"

First time connecting to a new server:

```bash
# Accept the host key
ssh-keyscan -t ed25519 gitea.yourdomain.com >> ~/.ssh/known_hosts

# Or connect manually first
ssh -T git@gitea.yourdomain.com
# Type "yes" when prompted
```

### Problem: Using wrong key

If you have multiple keys:

```bash
# Test which key is being offered
ssh -Tv git@gitea.yourdomain.com 2>&1 | grep "Offering public key"

# Force specific key
ssh -i ~/.ssh/id_ed25519 -T git@gitea.yourdomain.com

# Configure in ~/.ssh/config
Host gitea.yourdomain.com
    IdentityFile ~/.ssh/id_ed25519
```

### Problem: Key already added but still prompts for password

You might be using HTTPS URL instead of SSH:

```bash
# Check remote URL
git remote -v

# If you see https://... change to SSH:
git remote set-url gitea git@gitea.yourdomain.com:username/tui-slider.git

# Verify
git remote get-url gitea
```

## Verification Checklist

Run through this checklist to verify your setup:

```bash
# 1. SSH key exists
[ -f ~/.ssh/id_ed25519 ] && echo "✅ SSH key exists" || echo "❌ SSH key missing"

# 2. Public key exists
[ -f ~/.ssh/id_ed25519.pub ] && echo "✅ Public key exists" || echo "❌ Public key missing"

# 3. SSH connection works
ssh -T git@gitea.yourdomain.com 2>&1 | grep -q "successfully" && echo "✅ SSH connection works" || echo "❌ SSH connection failed"

# 4. Git remote uses SSH
git remote get-url gitea | grep -q "^git@" && echo "✅ Using SSH URL" || echo "❌ Using HTTPS URL"

# 5. Test push (dry-run, doesn't actually push)
git push gitea main --dry-run && echo "✅ Can push to Gitea" || echo "❌ Push failed"
```

## Security Best Practices

1. **Never share your private key** (`id_ed25519` without `.pub`)
   - Only share the public key (`id_ed25519.pub`)
   - Private key stays on your computer

2. **Use a passphrase** (optional but recommended)
   - Adds extra layer of security
   - Use ssh-agent to avoid typing it repeatedly

3. **Use Ed25519** instead of RSA
   - More secure with smaller keys
   - Faster operations

4. **Regular key rotation** (advanced)
   - Generate new keys annually
   - Remove old keys from Gitea

5. **Different keys per device** (optional)
   - Easier to revoke if device is lost
   - Better audit trail

## Common Commands

```bash
# Generate new key
ssh-keygen -t ed25519 -C "your_email@example.com"

# View public key
cat ~/.ssh/id_ed25519.pub

# Copy to clipboard (Linux with xclip)
cat ~/.ssh/id_ed25519.pub | xclip -selection clipboard

# Copy to clipboard (macOS)
cat ~/.ssh/id_ed25519.pub | pbcopy

# Test connection
ssh -T git@gitea.yourdomain.com

# Test with verbose output
ssh -Tv git@gitea.yourdomain.com

# List keys in ssh-agent
ssh-add -l

# Add key to ssh-agent
ssh-add ~/.ssh/id_ed25519

# Remove all keys from ssh-agent
ssh-add -D

# Check permissions (should be 600 for private key)
ls -l ~/.ssh/id_ed25519
chmod 600 ~/.ssh/id_ed25519  # Fix if needed
```

## Next Steps

Once SSH is configured:

1. **Set up Gitea remote:**
   ```bash
   ./scripts/setup-gitea.sh git@gitea.yourdomain.com:username/tui-slider.git
   ```

2. **Start using dual hosting:**
   ```bash
   just push-all  # Push to both GitHub and Gitea
   ```

3. **Create releases:**
   ```bash
   just release 0.2.0  # Releases to both platforms
   ```

## Resources

- [GitHub SSH Documentation](https://docs.github.com/en/authentication/connecting-to-github-with-ssh)
- [Gitea SSH Documentation](https://docs.gitea.io/en-us/usage/ssh-issues-troubleshooting/)
- [SSH.com - SSH Keys Explained](https://www.ssh.com/academy/ssh/key)
- [Ed25519 vs RSA](https://medium.com/risan/upgrade-your-ssh-key-to-ed25519-c6e8d60d3c54)

## Quick Reference Card

```
┌──────────────────────────────────────────────────────────┐
│                SSH Setup Quick Reference                  │
├──────────────────────────────────────────────────────────┤
│ Generate:  ssh-keygen -t ed25519 -C "email@example.com" │
│ View:      cat ~/.ssh/id_ed25519.pub                    │
│ Add:       Gitea → Settings → SSH Keys → Add Key        │
│ Test:      ssh -T git@gitea.yourdomain.com              │
│ Use:       git push gitea main (no password!)           │
└──────────────────────────────────────────────────────────┘
```

---

**Need help?** Check [GITEA_SETUP.md](./GITEA_SETUP.md) for the complete dual-hosting guide.