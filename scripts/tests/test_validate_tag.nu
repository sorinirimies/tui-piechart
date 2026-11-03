#!/usr/bin/env nu
# ── tui-piechart · test_validate_tag.nu ─────────────────────────────────────────
# Tests for scripts/ci/validate_tag.nu — tag validation and version extraction.

use std/assert
use runner.nu *
use ../ci/validate_tag.nu [validate]

# ── Tests ───────────────────────────────────────────────────────────────────

def "test validate_tag: simple release tag" [] {
    let result = (validate "v1.0.0")
    assert equal $result.tag "v1.0.0"
    assert equal $result.version "1.0.0"
}

def "test validate_tag: patch release" [] {
    let result = (validate "v0.1.6")
    assert equal $result.tag "v0.1.6"
    assert equal $result.version "0.1.6"
}

def "test validate_tag: zero version" [] {
    let result = (validate "v0.0.0")
    assert equal $result.tag "v0.0.0"
    assert equal $result.version "0.0.0"
}

def "test validate_tag: large version numbers" [] {
    let result = (validate "v12.345.6789")
    assert equal $result.tag "v12.345.6789"
    assert equal $result.version "12.345.6789"
}

def "test validate_tag: version strips v prefix" [] {
    let result = (validate "v3.2.1")
    assert (not ($result.version | str starts-with "v"))
}

def "test validate_tag: tag preserves v prefix" [] {
    let result = (validate "v3.2.1")
    assert ($result.tag | str starts-with "v")
}

def "test validate_tag: result has both keys" [] {
    let result = (validate "v1.0.0")
    let columns = ($result | columns)
    assert ("tag" in $columns)
    assert ("version" in $columns)
}

def "test validate_tag: rejects missing v prefix" [] {
    let failed = (try { validate "1.0.0"; false } catch { true })
    assert $failed
}

def "test validate_tag: rejects two-segment version" [] {
    let failed = (try { validate "v1.0"; false } catch { true })
    assert $failed
}

def "test validate_tag: rejects four-segment version" [] {
    let failed = (try { validate "v1.0.0.0"; false } catch { true })
    assert $failed
}

def "test validate_tag: rejects empty string" [] {
    let failed = (try { validate ""; false } catch { true })
    assert $failed
}

def "test validate_tag: rejects bare text" [] {
    let failed = (try { validate "release"; false } catch { true })
    assert $failed
}

def "test validate_tag: rejects pre-release suffix" [] {
    # The CI tag validator only accepts strict vX.Y.Z — no pre-release.
    let failed = (try { validate "v1.0.0-rc.1"; false } catch { true })
    assert $failed
}

def "test validate_tag: rejects alpha suffix" [] {
    let failed = (try { validate "v1.0.0-alpha"; false } catch { true })
    assert $failed
}

def "test validate_tag: rejects leading zeros in segments are still valid digits" [] {
    # v01.02.03 is technically valid digits, regex allows it since \d+ matches
    let result = (validate "v01.02.03")
    assert equal $result.version "01.02.03"
}

def "test validate_tag: multiple valid tags produce distinct results" [] {
    let a = (validate "v1.0.0")
    let b = (validate "v2.0.0")
    assert ($a.tag != $b.tag)
    assert ($a.version != $b.version)
}

def "test validate_tag: idempotent for same input" [] {
    let a = (validate "v5.5.5")
    let b = (validate "v5.5.5")
    assert equal $a.tag $b.tag
    assert equal $a.version $b.version
}

# ── Main ────────────────────────────────────────────────────────────────────

def main [] { run-tests }
