# Changelog

All notable changes to this project will be documented in this file.

## 0.2.3 - 2025-01-XX

### ✨ Features
- Add `LegendAlignment` enum with Left, Center, and Right options
- Add `.legend_alignment()` builder method to `PieChart`
- Support for aligning legend items within their allocated space
- Particularly useful for grid layouts and small chart areas

### 🐛 Bug Fixes
- Fix horizontal legend width calculation for Left/Right positions
- Fix legend text overflow when using horizontal layout with side positions
- Correct custom symbols example count from 12 to 8 throughout documentation

### 🎨 Improvements
- Apply centered legend alignment to all grid-layout examples
- Add centered alignment to: piechart, symbols_*, border_styles, custom_symbols
- Improve visual balance in 2x2 and 4x2 grid layouts
- Better padding in custom_symbols example

### 📚 Documentation
- Add comprehensive legend customization section to README
- Add `legend_alignment` example with 24 combinations demo
- Update EXAMPLES.md with legend alignment section
- Add VHS tape for legend_alignment demo
- Update all symbol example counts and descriptions
- Change main README preview to high_resolution.gif

### 🔧 Build System
- Add `run-legend-alignment` and `vhs-legend-alignment` justfile targets
- Update `generate_all.sh` with legend_alignment tape
- Update VHS README with new tape documentation

### 📊 Statistics
- Total combinations: 24 (4 positions × 2 layouts × 3 alignments)
- Examples: 12 (1 new: legend_alignment)
- VHS tapes: 12 (1 new: legend_alignment.tape)
- Tests: 70 passing (3 new for LegendAlignment)

**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.2.2...v0.2.3

## 0.2.2 - 2025-11-29
### ➕ Added
- Add publish readiness check script and Gitea pull commands
- Add border styles, legend, and title modules with examples
- Add setup-just.sh script for interactive just setup
### 📚 Documentation
- docs: add comprehensive examples section with all demo GIFs to README
### 📦 Other Changes
- Adds recipe for pulling from a Gitea repository
- Merge remote-tracking branch 'origin/main'
- Remove dual hosting and SSH setup documentation
- Reduce custom symbols demo from 12 to 8 examples
### 🔧 Chores
- chore: bump version to 0.2.0
- chore: bump version to 0.2.1
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.1.9...v0.2.2
## 0.1.9 - 2025-11-13
### ➕ Added
- Add comprehensive justfile and version bump script for automation
### 🔧 Chores
- chore: bump version to 0.1.8
- chore: bump version to 0.1.9
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.1.8...v0.1.9
## 0.1.8 - 2025-11-12
### 🔧 Chores
- chore: bump version to 0.1.8
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.1.7...v0.1.8
## 0.1.7 - 2025-11-12
### ➕ Added
- Add Gitea support and multi-remote release tasks
- Add Gitea dual-hosting setup guides and workflows
### 🐛 Bug Fixes
- fix justfile
### 📦 Other Changes
- Remove Windows from CI and release workflows
### 🔄 Updated
- Update README.md
- Update README.md
### 🔧 Chores
- chore: bump version to 0.1.7
**Full Changelog**: https://github.com/sorinirimies/tui-piechart/compare/v0.1.6...v0.1.7
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
