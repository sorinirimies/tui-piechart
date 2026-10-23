# Changelog

All notable changes to this project will be documented in this file.

## 1.1.1 - 2026-10-23
### 🔧 Chores
- chore(justfile): skip unconfigured remotes in *-all recipes
- chore(justfile): push/pull current branch instead of hardcoded main
- chore: update deps, fix clippy lints, add auto-merge workflow
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v1.1.0...v1.1.1
## 1.1.0 - 2026-10-14
### ✨ Features
- feat: shared theming via tui-nuances
- feat: make theming an optional "theme" feature
### 🐛 Bug Fixes
- fix(deps): revert accidental ratatui 0.30 -> 0.29 downgrade
- fix(ci): make Gitea deps-update commit-only, matching GitHub mirror
- fix(scripts): drop deprecated str downcase; exit non-zero when bump is aborted
### 📚 Documentation
- docs(readme): restyle crates.io version badge
- docs: re-record demos showing results only; reference tui-nuances version
### 📦 Other Changes
- Merge pull request #5 from sorinirimies/deps/nightly-dependency-upgrade
- justfile: add gitea-nexus-lab remote recipes (push/pull/sync/release)
- justfile: rename gitea -> gitea-microlab; add full gitea-microlab/-starscream/-nexus-lab parity
- Add/complete nightly dependency-update automation (Gitea): update deps, quality-gate, bump patch version, tag, push to Gitea + GitHub
- Merge gitea-starscream/main
### 🔄 CI
- ci: grant contents/pull-requests write perms to deps-update workflow
- ci(deps-update): guard against silent dependency downgrades
- ci(deps-update): move downgrade guard into shared Nushell script
- ci(deps): release only on real Cargo.toml dependency changes; lock-only updates commit without release
- ci(security): add cargo-audit workflow and just audit recipe
- ci: prebuilt Nushell, cargo-deny + cargo-audit security workflow; add cargo-fuzz
### 🔧 Chores
- chore(deps): nightly dependency upgrade
- chore(deps): nightly dependency upgrade 2026-05-21
- chore(deps): nightly dependency upgrade 2026-09-09
- chore(deps): nightly dependency upgrade 2026-09-10
- chore(deps): nightly dependency upgrade 2026-09-12
- chore(deps): nightly dependency upgrade 2026-09-17
- chore(deps): nightly dependency upgrade 2026-09-23
- chore(deps): nightly dependency upgrade 2026-09-26
- chore(deps): nightly dependency upgrade 2026-10-01
- chore(deps): nightly dependency upgrade 2026-10-03
- chore(deps): nightly dependency upgrade 2026-10-04
- chore(just): add force-push recipes (per-remote, tags, push-all-force-with-tags)
- chore: bump version to 1.0.3
- chore: depend on tui-nuances 0.1.5
- chore: depend on tui-nuances 0.1.6
- chore: depend on tui-nuances 0.1.6 from crates.io
- chore: bump version to 1.1.0
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v1.0.2...v1.1.0
## 1.0.2 - 2026-08-05
### 📚 Documentation
- docs: add crates.io downloads badge to README
### 📦 Other Changes
- ignore codegraph setup
- Merge pull request #3 from SomethingSomehow/main
### 🔄 Updated
- update ratatui to 0.30
### 🔧 Chores
- chore: bump version to 1.0.2
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v1.0.1...v1.0.2
## 1.0.1 - 2026-07-16
### 🔧 Chores
- chore: bump version to 1.0.1
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v1.0.0...v1.0.1
## 1.0.0 - 2026-07-16
### ♻️ Refactor
- refactor: migrate shell scripts to Nushell
- refactor: inline VHS generation into justfile, remove generate_all.sh
### ✨ Features
- feat: add nightly dependency upgrade workflow
### 🐛 Bug Fixes
- fix: add Stylize trait import to all examples for ratatui 0.29 compat
- fix: render a 100% pie slice as a filled disc instead of a line (#2)
### 📦 Other Changes
- Make title_vertical_position a no-op for ratatui 0.30
- Remove contribution and Gitea setup docs, update justfile tasks
- Reorder Stylize import and allow_unused_imports in examples
- Simplify deps-update workflow to push directly to main
### 🔄 Updated
- Update README with author credit and remove extra sections
- Update dependencies and fix Gitea PR head filter format
- Update README installation instructions and bump_version script
### 🔧 Chores
- chore: update rules and widen dep-update commit scope
- chore: bump version to 0.3.2
- chore: bump version to 0.3.3
- chore(deps): nightly dependency upgrade 2026-04-27
- chore: bump version to 1.0.0
### 🧪 Testing
- test: add matches_test! macro and consolidate legend enum tests
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.3.1...v1.0.0
## 0.3.1 - 2026-02-26
### 📦 Other Changes
- added zed rules
### 🔧 Chores
- chore: bump version to 0.3.1
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.3.0...v0.3.1
## 0.3.0 - 2026-02-20
### 🐛 Bug Fixes
- fix(warnings): remove unused Stylize imports from all examples
- fix(warnings): remove unnecessary braces around macro arguments
### 🔧 Chores
- chore: bump version to 0.3.0
### 🧪 Testing
- test: achieve 100% coverage (95.54% -> 100%)
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.2.9...v0.3.0
## 0.2.9 - 2026-02-20
### ➕ Added
- Add checks and templates for git pull/push commands in justfile
- Add Git pull commands for origin and gitea remotes
### 🐛 Bug Fixes
- fix: sed -i macOS compatibility in bump_version.sh
### 📦 Other Changes
- Exclude CI, setup files, and example GIFs from package
- Merge remote-tracking branch 'origin/main'
- Clean up conditional formatting in key event handlers
- Reduce custom symbols example from 8 to 6 charts
- Remove legacy visual previews section from README
- Auto-discover VHS tapes in generate_all.sh and update vhs-all task
### 🔄 Updated
- Update custom_symbols.gif with new output
- Update custom symbols description in README
- Update dependencies in Cargo.lock
### 🔧 Chores
- chore: bump version to 0.2.5
- chore: bump version to 0.2.6
- chore: bump version to 0.2.7
- chore: bump version to 0.2.8
- chore: upgrade dependencies to latest versions
- chore: bump version to 0.2.9
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.2.4...v0.2.9
## 0.2.4 - 2025-11-30
### ♻️ Refactor
- Refactor legend rendering into helper methods
### ➕ Added
- Add legend marker showcase and unify title styles demo
- Add legend alignment feature and update examples
### 🔧 Chores
- chore: bump version to 0.2.4
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.2.2...v0.2.4
## 0.2.2 - 2025-11-29
### ➕ Added
- Add Gitea support and multi-remote release tasks
- Add Gitea dual-hosting setup guides and workflows
- Add comprehensive justfile and version bump script for automation
- Add publish readiness check script and Gitea pull commands
- Add border styles, legend, and title modules with examples
- Add setup-just.sh script for interactive just setup
### 🐛 Bug Fixes
- fix justfile
### 📚 Documentation
- docs: add comprehensive examples section with all demo GIFs to README
### 📦 Other Changes
- Remove Windows from CI and release workflows
- Adds recipe for pulling from a Gitea repository
- Merge remote-tracking branch 'origin/main'
- Remove dual hosting and SSH setup documentation
- Reduce custom symbols demo from 12 to 8 examples
### 🔄 Updated
- Update README.md
- Update README.md
### 🔧 Chores
- chore: bump version to 0.1.7
- chore: bump version to 0.1.8
- chore: bump version to 0.1.8
- chore: bump version to 0.1.9
- chore: bump version to 0.2.0
- chore: bump version to 0.2.1
- chore: bump version to 0.2.2
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.1.6...v0.2.2
## 0.1.6 - 2025-11-09
### 📦 Other Changes
- Expand categories in Cargo.toml
### 🔧 Chores
- chore: bump version to 0.1.6
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.1.5...v0.1.6
## 0.1.5 - 2025-11-09
### 📦 Other Changes
- Move VHS examples to vhs/ and update output paths
- Merge remote-tracking branch 'origin/main'
### 🔄 Updated
- Update README.md
### 🔧 Chores
- chore: bump version to 0.1.5
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.1.4...v0.1.5
## 0.1.4 - 2025-11-09
### ➕ Added
- Add individual VHS demo tasks and vhs-all aggregate task
### 📦 Other Changes
- Revamp piechart example with animated charts and controls
### 🔧 Chores
- chore: bump version to 0.1.4
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.1.3...v0.1.4
## 0.1.3 - 2025-11-08
### 📦 Other Changes
- Remove extra blank lines before main functions in examples
### 🔧 Chores
- chore: bump version to 0.1.2
- chore: bump version to 0.1.3
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.1.2...v0.1.3
## 0.1.2 - 2025-11-08
### ➕ Added
- Add pie chart widget, examples, CI, and docs
- Add high resolution mode with braille rendering
### 📦 Other Changes
- Initial commit
### 🔧 Chores
- chore: bump version to 0.1.2
