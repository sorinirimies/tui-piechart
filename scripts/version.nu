#!/usr/bin/env nu
# Prints the current package version from Cargo.toml.
# Usage: nu scripts/version.nu

open Cargo.toml | get package.version | print
