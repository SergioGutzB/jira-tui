# Release Instructions

This document describes how to create a new release of Jira TUI.

## Prerequisites

1. All changes are committed and pushed to `main`
2. CI is passing on GitHub Actions
3. You have push access to the repository
4. Git is configured correctly

## Release Process

### 1. Update Version

Update the version in `Cargo.toml`:

```toml
[package]
version = "0.1.0-beta.1"  # Change this
```

### 2. Update RELEASE_NOTES.md

Document changes in `RELEASE_NOTES.md`:
- New features
- Bug fixes
- Breaking changes
- Known issues

### 3. Commit Version Bump

```bash
git add Cargo.toml RELEASE_NOTES.md
git commit -m "chore: bump version to v0.1.0-beta.1"
git push origin main
```

### 4. Create Git Tag

```bash
# Create annotated tag
git tag -a v0.1.0-beta.1 -m "Release v0.1.0-beta.1"

# Push tag to GitHub
git push origin v0.1.0-beta.1
```

### 5. GitHub Actions Will Automatically:

Once you push the tag, GitHub Actions will:
1. Build binaries for all platforms (Linux x86_64, Linux musl, macOS x86_64, macOS aarch64)
2. Create a GitHub Release
3. Upload all binaries to the release
4. Mark as pre-release if version contains "beta" or "alpha"

### 6. Edit Release Notes on GitHub

1. Go to https://github.com/YOUR_USERNAME/jira-tui/releases
2. Find the new release (auto-created by GitHub Actions)
3. Edit the release description if needed
4. Add any additional notes or highlights

### 7. Announce

- Tweet/post about the release
- Update project website (if any)
- Notify users in discussions/Discord/etc.

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- `MAJOR.MINOR.PATCH`
- `MAJOR`: Breaking changes
- `MINOR`: New features (backward compatible)
- `PATCH`: Bug fixes (backward compatible)

### Pre-release Versions

- `0.1.0-alpha.1`: Early development, unstable
- `0.1.0-beta.1`: Feature complete, testing phase
- `0.1.0-rc.1`: Release candidate, final testing
- `0.1.0`: Stable release

## Rollback a Release

If something goes wrong:

```bash
# Delete local tag
git tag -d v0.1.0-beta.1

# Delete remote tag
git push --delete origin v0.1.0-beta.1

# Delete GitHub Release manually on GitHub
```

Then fix the issues and create a new tag/release.

## Manual Binary Build (if needed)

If you need to build binaries manually:

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# Linux musl (static)
cargo build --release --target x86_64-unknown-linux-musl

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin
```

Binaries will be in `target/<TARGET>/release/jira-tui`.

## Testing Before Release

Before creating a tag:

```bash
# Run all tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings

# Build release binary
cargo build --release

# Test the binary
./target/release/jira-tui
```

## Post-Release Checklist

- [ ] Release created on GitHub
- [ ] All binaries uploaded and working
- [ ] Release notes are clear and complete
- [ ] Documentation is up to date
- [ ] Announcement made (if applicable)
- [ ] Version bumped in `main` for next development cycle

## Example: Complete Release Flow

```bash
# 1. Ensure you're on main and up to date
git checkout main
git pull origin main

# 2. Update version in Cargo.toml (manually)
# version = "0.1.0-beta.1"

# 3. Update RELEASE_NOTES.md (manually)

# 4. Commit version bump
git add Cargo.toml RELEASE_NOTES.md
git commit -m "chore: bump version to v0.1.0-beta.1"

# 5. Create and push tag
git tag -a v0.1.0-beta.1 -m "Release v0.1.0-beta.1"
git push origin main
git push origin v0.1.0-beta.1

# 6. Wait for GitHub Actions to complete (check Actions tab)

# 7. Go to Releases page and verify
# https://github.com/YOUR_USERNAME/jira-tui/releases

# 8. Download and test one binary to verify it works

# Done! 🎉
```

## Troubleshooting

### GitHub Actions fails

- Check logs in Actions tab
- Common issues:
  - Missing secrets (should not be needed for public repos)
  - Permission issues (check workflow permissions)
  - Build errors (test locally first)

### Binary doesn't work

- Check that it's built for the correct architecture
- Verify dynamic library dependencies (`ldd` on Linux, `otool -L` on macOS)
- Check file permissions (`chmod +x`)

### Release not created

- Verify tag format matches pattern in `.github/workflows/release.yml`
- Check that tag was pushed to GitHub
- Verify workflow permissions in repository settings

## Questions?

If you have questions about the release process, open a discussion or issue on GitHub.
