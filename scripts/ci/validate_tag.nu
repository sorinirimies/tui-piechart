#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-piechart — Validate release tag
# ──────────────────────────────────────────────────────────────────────────────
# Validates that a tag matches the vX.Y.Z pattern and outputs the tag and the
# bare version string.  Used by CI release workflows so validation logic lives
# in one place rather than being duplicated across GitHub / Gitea YAML.
#
# Usage:
#   nu scripts/ci/validate_tag.nu <tag>
#
# Examples:
#   nu scripts/ci/validate_tag.nu v0.5.0
#   nu scripts/ci/validate_tag.nu v1.2.3
#
# On success, prints the tag and version and exits 0.
# On failure, prints an error and exits 1.
#
# When running inside GitHub / Gitea Actions the caller should capture the
# output and write it to $GITHUB_OUTPUT:
#
#   RESULT=$(nu scripts/ci/validate_tag.nu "$TAG")
#   echo "$RESULT" >> "$GITHUB_OUTPUT"
# ──────────────────────────────────────────────────────────────────────────────

# Validate a tag string and return the tag + bare version.
#
# On invalid input this raises a catchable error (via `error make`) so that
# callers — including test harnesses — can intercept it with `try / catch`.
export def validate [tag: string]: nothing -> record<tag: string, version: string> {
    if ($tag | is-empty) {
        error make {
            msg: $"Tag is empty — nothing to validate."
        }
    }

    let pattern = '^v\d+\.\d+\.\d+$'
    if ($tag | find --regex $pattern | is-empty) {
        error make {
            msg: $"Tag '($tag)' does not match vX.Y.Z — aborting."
        }
    }

    let version = ($tag | str replace 'v' '')

    { tag: $tag, version: $version }
}

# ── Main ──────────────────────────────────────────────────────────────────────
# When invoked directly from the command line (or CI), validation errors should
# terminate the process with a non-zero exit code and a human-friendly message.

def main [tag: string] {
    let result = try {
        validate $tag
    } catch { |err|
        print --stderr $"(ansi red)❌ ($err.msg)(ansi reset)"
        exit 1
    }

    # Human-friendly message goes to stderr so it doesn't pollute $GITHUB_OUTPUT.
    print --stderr $"(ansi green)✅ Tag ($result.tag) \(version ($result.version)\) is valid.(ansi reset)"

    # Only key=value lines go to stdout — the caller appends directly:
    #   nu scripts/ci/validate_tag.nu "$TAG" >> "$GITHUB_OUTPUT"
    print $"tag=($result.tag)"
    print $"version=($result.version)"
}
