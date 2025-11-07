# Contributing to tui-piechart

Thank you for your interest in contributing to tui-piechart! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- Rust 1.74.0 or later
- [just](https://github.com/casey/just) - command runner (optional but recommended)
- [git-cliff](https://github.com/orhun/git-cliff) - changelog generator (optional)

### Getting Started

1. Fork and clone the repository:
```bash
git clone https://github.com/yourusername/tui-piechart
cd tui-piechart
```

2. Install development tools (optional):
```bash
just install-tools
```

3. Run tests to ensure everything works:
```bash
just test
# or
cargo test
```

## Development Workflow

### Running the Example

```bash
just run
# or
cargo run --example piechart
```

### Running Tests

```bash
just test
# or
cargo test
```

### Code Formatting

We use `rustfmt` for code formatting:

```bash
just fmt
# or
cargo fmt
```

Check formatting without modifying files:

```bash
just fmt-check
# or
cargo fmt --check
```

### Linting

We use `clippy` for linting:

```bash
just clippy
# or
cargo clippy -- -D warnings
```

### Running All Checks

Run all checks (formatting, linting, tests):

```bash
just check-all
```

## Making Changes

### Code Style

- Follow Rust naming conventions
- Use `rustfmt` for formatting (max line width: 100)
- Address all `clippy` warnings
- Add documentation for public APIs
- Include examples in documentation

### Testing

- Add tests for new functionality
- Ensure all tests pass before submitting
- Aim for high test coverage

### Documentation

- Document all public APIs with doc comments
- Include examples in doc comments
- Update README.md if adding new features
- Update CHANGELOG.md (or it will be auto-generated)

### Commit Messages

We follow conventional commit format:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions or changes
- `refactor:` - Code refactoring
- `style:` - Code style changes
- `chore:` - Maintenance tasks
- `ci:` - CI/CD changes

Example:
```
feat: add new pie chart rendering mode

Adds support for displaying pie charts with customizable slice angles.

Closes #123
```

## Submitting Changes

1. Create a new branch:
```bash
git checkout -b feature/my-feature
```

2. Make your changes and commit:
```bash
git add .
git commit -m "feat: add new feature"
```

3. Push to your fork:
```bash
git push origin feature/my-feature
```

4. Open a Pull Request on GitHub

### Pull Request Guidelines

- Keep PRs focused on a single change
- Include tests for new functionality
- Update documentation as needed
- Ensure all CI checks pass
- Link related issues

## Questions?

Feel free to open an issue for questions or discussions!

## License

By contributing to tui-piechart, you agree that your contributions will be licensed under the MIT License.
