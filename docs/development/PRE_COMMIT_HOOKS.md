# 🔧 Pre-commit Hooks Setup

This document explains how to set up and use pre-commit hooks for the leptos-state project to ensure code quality and consistency.

## 🚀 Quick Setup

### Option 1: Using Make (Recommended)
```bash
make setup-hooks
```

### Option 2: Manual Setup
```bash
./scripts/setup-pre-commit.sh
```

### Option 3: Direct Installation
```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install hooks
pre-commit install
pre-commit install --hook-type commit-msg
```

## 📋 What Gets Installed

### Pre-commit Framework Hooks
- **Rustfmt**: Code formatting with Rust 2024 edition
- **Clippy**: Linting with strict warnings
- **General Hooks**: Trailing whitespace, file endings, YAML/JSON validation
- **Documentation**: Prettier for markdown files
- **Security**: Detect-secrets for credential scanning

### Custom Git Hooks
- **Pre-commit**: Runs cargo check, test, clippy, and fmt
- **Commit-msg**: Validates commit message format
- **Version Consistency**: Ensures version numbers match across files
- **Documentation Links**: Validates markdown links

## 🔍 Hook Details

### Pre-commit Hook
Runs automatically before each commit:
- ✅ `cargo check` - Compilation check
- ✅ `cargo test` - Unit tests
- ✅ `cargo clippy` - Linting with warnings as errors
- ✅ `cargo fmt --check` - Formatting validation
- ✅ Large file detection (>1MB)
- ✅ Merge conflict marker detection
- ✅ Version consistency check

### Commit Message Hook
Validates commit messages:
- ✅ Minimum 10 characters
- ✅ Maximum 72 characters for first line
- ✅ No trailing whitespace
- ✅ No multiple empty lines
- ⚠️ Suggests conventional commit format

### Version Consistency Hook
Ensures version numbers match in:
- `leptos-state/Cargo.toml`
- `README.md`
- `docs/ROADMAP.md`
- `docs/CHANGELOG.md`
- `docs/api-reference/API_REFERENCE.md`

## 🛠️ Usage

### Running Hooks Manually
```bash
# Run on all files
pre-commit run --all-files

# Run on staged files only
pre-commit run

# Run specific hook
pre-commit run rustfmt
pre-commit run clippy
```

### Skipping Hooks (Not Recommended)
```bash
# Skip all hooks
git commit --no-verify -m "message"

# Skip specific hook
SKIP=clippy git commit -m "message"
```

### Updating Hooks
```bash
# Update hook versions
pre-commit autoupdate

# Reinstall hooks
pre-commit uninstall
pre-commit install
```

## 🔧 Configuration

### Pre-commit Config
The main configuration is in `.pre-commit-config.yaml`:
- Hook repositories and versions
- File patterns and exclusions
- Custom local hooks
- Failure behavior

### Custom Hooks
Located in `.git/hooks/`:
- `pre-commit`: Main pre-commit logic
- `commit-msg`: Commit message validation
- `check-version-consistency`: Version checking
- `check-documentation-links`: Link validation

## 🚨 Troubleshooting

### Common Issues

#### Pre-commit Not Found
```bash
# Install pre-commit
pip install pre-commit
# or
brew install pre-commit
```

#### Hook Failures
```bash
# Check what failed
pre-commit run --all-files

# Fix formatting issues
cargo fmt

# Fix clippy issues
cargo clippy --fix
```

#### Version Mismatch
```bash
# Check current version in Cargo.toml
grep version leptos-state/Cargo.toml

# Update version in all files
# (Manual process - update each file)
```

#### Large Files
```bash
# Check file sizes
git ls-files | xargs -I {} sh -c 'echo "{} $(wc -c < "{}")"' | sort -k2 -nr

# Add to .gitignore if appropriate
echo "large-file.bin" >> .gitignore
```

### Disabling Hooks Temporarily
```bash
# Disable all hooks
pre-commit uninstall

# Re-enable hooks
pre-commit install
```

## 📚 Best Practices

### Commit Messages
Use conventional commit format:
```
feat: add new state machine feature
fix: resolve memory leak in persistence
docs: update API documentation
test: add integration tests for guards
```

### Code Quality
- Always run `cargo fmt` before committing
- Fix clippy warnings before committing
- Ensure all tests pass
- Keep files under 1MB

### Version Management
- Update version in `leptos-state/Cargo.toml` first
- Update all documentation files
- Run version consistency check

## 🔄 Integration with CI/CD

The pre-commit hooks complement our CI/CD pipeline:
- Pre-commit catches issues early
- CI runs the same checks in isolation
- Both ensure consistent code quality

## 📖 Additional Resources

- [Pre-commit Documentation](https://pre-commit.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Rust Clippy](https://doc.rust-lang.org/clippy/)
- [Rustfmt](https://github.com/rust-lang/rustfmt)

---

*Last Updated: September 2025*
