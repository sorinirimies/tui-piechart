#!/usr/bin/env nu
# Prints the current workspace version from Cargo.toml.
# Usage: nu scripts/version.nu

export def read_version [] {
    cargo_version (open Cargo.toml)
}

# The version of a parsed Cargo.toml: `[workspace.package]` (a workspace) or
# `[package]` (a single crate at the root).
export def cargo_version [cargo: record]: nothing -> string {
    let ws = $cargo | get -o workspace.package.version
    if $ws != null { $ws } else { $cargo | get package.version }
}

def main [] { print (read_version) }
