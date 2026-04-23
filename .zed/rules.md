# Zed Editor Rules for tui-piechart

## Project Overview

**tui-piechart** is a Rust library providing customizable pie chart widgets for Ratatui TUI applications. The project emphasizes:

- Zero-cost abstractions and performance
- Terminal UI widget for data visualization
- Extensive customization (colors, symbols, legends, borders, high-resolution rendering)
- Comprehensive examples and documentation
- Clean, modular architecture

## Code Style & Standards

### Rust Standards
- **Rust Version**: 1.74.0 minimum (MSRV)
- **Edition**: 2021
- **Max Line Width**: 100 characters (see `rustfmt.toml`)
- **Linting**: Clippy pedantic with `-D warnings` (fails on warnings)
- **Documentation**: All public APIs must have doc comments with examples

### Code Organization
```
src/
├── lib.rs           # Main entry point, PieChart widget implementation
├── border_style.rs  # Border style definitions
├── legend.rs        # Legend positioning, layout, alignment
├── symbols.rs       # Unicode symbol constants
├── title.rs         # Title positioning and styling
└── macros/          # Internal macros
```

### Naming Conventions
- Use descriptive names that match Rust conventions
- Public APIs use `snake_case` for functions/methods
- Types use `PascalCase`
- Constants use `SCREAMING_SNAKE_CASE` (e.g., `PIE_CHAR_BLOCK`)
- Builder pattern methods (fluent API): `show_legend()`, `pie_char()`, etc.

### Documentation Requirements
- Public items require doc comments with:
  - Brief description
  - Code examples using ` ```rust ` or ` ```no_run `
  - Parameter explanations if not self-evident
- Use emoji consistently in feature lists (🥧, 🎨, 🔤, 📊, 📦, ✨, ⚡)
- Link to related types/modules using `[Type]`

### Code Quality Rules
1. **Zero unsafe code** - project uses only safe Rust
2. **No unwrap/expect** in library code - handle errors gracefully
3. **Builder pattern** - use for configuration (see `PieChart`)
4. **Trait implementations** - implement `Default`, `Styled`, etc. where appropriate
5. **Module re-exports** - common types exported at crate root for convenience

## Formatting & Linting

### Before Committing
```bash
just fmt          # Format code (required)
just clippy       # Lint with clippy (must pass with -D warnings)
just test         # Run test suite (must pass)
just check-all    # Run all checks
```

### Auto-format on Save
- Enable rustfmt on save in Zed
- Configuration: `rustfmt.toml` (max_width=100, edition="2021")

### Clippy Configuration
- Pedantic mode enabled: `#![warn(clippy::pedantic)]`
- Allowed: `#![allow(clippy::module_name_repetitions)]`
- Additional lints in source files: `#![warn(missing_docs)]`

## Git Workflow

### Branch Naming
- `feature/<name>` - New features
- `fix/<name>` - Bug fixes
- `docs/<name>` - Documentation updates
- `refactor/<name>` - Code refactoring
- `test/<name>` - Test additions/changes
- `chore/<name>` - Maintenance tasks

### Commit Message Format

**Use conventional commits** (for changelog generation via git-cliff):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Commit Types (match cliff.toml)
- `feat:` - ✨ New features
- `fix:` - 🐛 Bug fixes
- `docs:` - 📚 Documentation changes
- `perf:` - ⚡ Performance improvements
- `refactor:` - ♻️ Code refactoring (no functional changes)
- `style:` - 💄 Formatting/style changes
- `test:` - 🧪 Test additions/changes
- `chore:` - 🔧 Maintenance tasks (deps, configs)
- `build:` - 📦 Build system changes
- `ci:` - 🔄 CI/CD changes

#### Examples
```bash
feat: add braille rendering mode for high resolution

Add Resolution::Braille variant that uses Unicode braille patterns
to achieve 8x resolution improvement over standard rendering.

Closes #42

---

fix: prevent legend text cutoff with long labels

Calculate required legend width dynamically based on actual content
to ensure text is never truncated.

---

docs: add interactive demo GIFs to README

Include VHS-generated demos for all 11 example programs.

---

test: add unit tests for legend layout calculations

---

chore: update ratatui to 0.30
```

### Breaking Changes
Mark breaking changes explicitly:
```
feat!: change PieChart API to use builder pattern

BREAKING CHANGE: PieChart::new() now returns builder instead of 
configured widget. Call .build() to finalize.
```

## Auto-Commit Rules

- After any code fix (including dependency adaptation), always run `just check-all` before committing.
- The `just update-deps*` recipes and `scripts/upgrade_deps.nu` will automatically commit and push all staged changes (Cargo files + source fixes) if the quality gate passes.
- The `just commit "message"` recipe stages all files (`git add .`) and commits — use it for quick single-purpose commits.
- Always use conventional commit messages (see Commit Message Format above).
- The `.zed/rules.md` file itself is tracked in git and should be committed whenever it changes.

## Pull Request Guidelines

### PR Structure
1. **Title**: Use conventional commit format
2. **Description**: 
   - What: Describe the change
   - Why: Explain the motivation
   - How: Outline the approach
3. **Testing**: Describe how you tested
4. **Screenshots**: Include for visual changes
5. **Checklist**:
   - [ ] Code formatted (`just fmt`)
   - [ ] Clippy passes (`just clippy`)
   - [ ] Tests pass (`just test`)
   - [ ] Documentation updated
   - [ ] Examples added/updated if needed

### PR Size
- Keep PRs focused on a single concern
- Prefer multiple small PRs over one large PR
- Split unrelated changes into separate PRs

### Review Requirements
- All CI checks must pass (formatting, clippy, tests)
- At least one maintainer approval
- Address all review comments or explain disagreement
- Keep discussions respectful and constructive

## Testing Guidelines

### Test Coverage
- Unit tests for business logic
- Integration tests in `tests/` directory
- Examples serve as integration tests (must compile and run)

### Running Tests
```bash
just test                 # Run all tests
cargo test                # Direct cargo invocation
cargo test --lib          # Library tests only
cargo test --examples     # Ensure examples compile
```

### Test Naming
```rust
#[test]
fn test_pie_chart_calculates_angles_correctly() { }

#[test]
fn test_legend_layout_vertical_spacing() { }
```

## Examples & Documentation

### Example Programs
Located in `examples/` directory:
- Each example must be interactive and self-documenting
- Include keyboard navigation help text
- Use descriptive names: `legend_positioning.rs`, `high_resolution.rs`
- Register in `Cargo.toml` `[[example]]` section

### Running Examples
```bash
just run-<example-name>      # Via just
cargo run --example <name>   # Via cargo
```

### Documentation Standards
- Keep README.md as the main entry point
- EXAMPLES.md for detailed example documentation
- CONTRIBUTING.md for contributor guidelines
- Doc comments in source files for API docs

## Dependencies

### Core Dependencies
- `ratatui = "0.30"` - Core TUI framework
- No other runtime dependencies (keep it minimal)

### Dev Dependencies
- `color-eyre = "0.6"` - Error handling in examples
- `crossterm = "0.29"` - Terminal backend for examples

### Dependency Management
```bash
cargo update              # Update dependencies
cargo outdated            # Check for outdated deps
just update               # Via just command
```

## Dependency Upgrade Handling

### Breaking API Changes
When a nightly dependency upgrade introduces breaking API changes (e.g., a trait method becomes inherent, a type is renamed, an import path changes), fix the source code to match the new API rather than pinning the old version. Common patterns include:
- **Missing trait imports**: e.g., `use ratatui::style::Stylize` was needed in 0.29 but not 0.30 — remove the import if the trait method became inherent.
- **Renamed or moved types**: update all `use` statements and type references throughout `src/` and `examples/`.
- **Changed method signatures**: adapt call sites to the new signature.

### Quality Gate Failures
When `cargo clippy`, `cargo test`, or `cargo fmt --check` fail after a dep update, investigate the root cause. If it's a straightforward API migration, fix the code and re-run the gate.

### Commit Scope
When fixing breakage from a dependency upgrade, commit **all** affected files — not just `Cargo.toml` and `Cargo.lock` but also any modified source files in `src/` and `examples/`. Use the commit message format:
- `fix: adapt to <crate> <version> API changes`
- `chore(deps): update for <crate> <version> compatibility`

### Automated Workflows
The nightly CI workflows (`.gitea/workflows/deps-update.yml`, `.github/workflows/deps-update.yml`) run `cargo upgrade --incompatible allow` + quality gate. If the gate fails, the PR is not created. Developers must then fix the source manually, run `just check-all`, and commit everything together.

### Local Dep Update
Use `just update-deps` (or the remote-specific variants) which runs the quality gate and commits all changes. Always verify examples compile with `cargo check --examples` after upgrading.

## Release Process

### Version Bumping
```bash
just bump <version>       # e.g., just bump 0.4.0
```

This script (`scripts/bump_version.sh`):
1. Updates version in `Cargo.toml`
2. Runs `check-all` (fmt, clippy, test)
3. Generates changelog with git-cliff
4. Creates git tag `v<version>`
5. Commits changes

### Changelog Generation
Managed by git-cliff (see `cliff.toml`):
```bash
just changelog-preview              # Preview without writing
just changelog-unreleased           # Add unreleased changes
just changelog                      # Regenerate full changelog
```

### Publishing
```bash
just publish-dry         # Dry run to verify
just publish             # Publish to crates.io
```

### Release Workflow
```bash
# Full release to GitHub
just release <version>

# Or manual steps:
just bump <version>
git push origin main
git push origin v<version>

# GitHub Actions handles:
# - Running CI checks
# - Creating GitHub release
# - Publishing to crates.io (if configured)
```

## Project-Specific Guidelines

### Widget Implementation
- Use Ratatui's `Widget` trait
- Implement `Styled` for style customization
- Use builder pattern for configuration
- Calculate layout dynamically based on available space

### Symbol Customization
- Provide predefined constants in `symbols` module
- Allow any Unicode character via `.pie_char(char)`
- Allow any string via `.legend_marker(&str)`
- Document terminal compatibility considerations

### Performance
- Avoid allocations in hot paths (rendering)
- Use iterators over collecting intermediate vectors
- Profile with `cargo flamegraph` for optimization work
- Benchmark before claiming performance improvements

### Terminal Compatibility
- Test with common terminals (iTerm2, Terminal.app, Alacritty, Windows Terminal)
- Document Unicode character support requirements
- Provide ASCII fallbacks where reasonable
- Consider color support levels (256 color, true color)

### High Resolution Rendering
- Braille pattern rendering (8x resolution improvement)
- Document performance characteristics
- Provide toggle examples (standard vs. braille)

## Common Tasks

### Quick Reference
```bash
# Development
just run                  # Run main example
just test                 # Run tests
just check-all            # Full check before commit

# Code Quality
just fmt                  # Format code
just clippy               # Lint code
just doc                  # Generate & open docs

# Examples
just run-<tab>           # Tab completion for examples
just run-high-resolution # High-res demo
just run-border-styles   # Border styles showcase

# Git Operations
just commit "msg"        # Quick commit
just push                # Push to GitHub
just push-all            # Push to GitHub & Gitea

# Release
just bump 0.4.0          # Bump version
just release 0.4.0       # Full release workflow
```

## Code Review Checklist

### For Reviewers
- [ ] Code follows project style and conventions
- [ ] Public APIs have documentation with examples
- [ ] Tests cover new functionality
- [ ] No unwrap/expect in library code
- [ ] Breaking changes are clearly marked
- [ ] Examples compile and run correctly
- [ ] Performance impact is acceptable
- [ ] Unicode/terminal compatibility considered

### For Authors
- [ ] Self-review completed
- [ ] Tested manually with example programs
- [ ] CI passes locally (`just check-all`)
- [ ] Git history is clean (consider squashing)
- [ ] Commit messages follow conventional format
- [ ] Documentation updated
- [ ] Breaking changes documented in commit footer

## Anti-Patterns to Avoid

### Code
- ❌ Using `unwrap()` or `expect()` in library code
- ❌ Hardcoded dimensions or magic numbers without explanation
- ❌ Panicking in rendering code
- ❌ Allocating in hot paths without profiling first
- ❌ Adding dependencies without discussion

### Git
- ❌ Commit messages without type prefix
- ❌ Mixing unrelated changes in one commit
- ❌ Force pushing to main branch
- ❌ Committing without running tests
- ❌ Large commits without explanation

### Documentation
- ❌ Public APIs without doc comments
- ❌ Examples without keyboard help text
- ❌ Missing examples for new features
- ❌ Outdated documentation after changes

## Resources

### Project Resources
- **Repository**: https://github.com/sorinirimies/tui-piechart
- **Documentation**: https://docs.rs/tui-piechart
- **Crates.io**: https://crates.io/crates/tui-piechart

### Rust Resources
- **Ratatui**: https://github.com/ratatui/ratatui
- **Rust Book**: https://doc.rust-lang.org/book/
- **Clippy Lints**: https://rust-lang.github.io/rust-clippy/

### Tools
- **just**: https://github.com/casey/just
- **git-cliff**: https://github.com/orhun/git-cliff
- **VHS**: https://github.com/charmbracelet/vhs (demo GIFs)

## Questions & Support

- Open an issue for bugs or feature requests
- Check CONTRIBUTING.md for detailed guidelines
- Review existing PRs for examples
- Ask questions in issues before starting large changes

---

**Last Updated**: 2025-07-17
**Project Version**: 0.3.1
**Maintainer**: Sorin Albu-Irimies (@sorinirimies)