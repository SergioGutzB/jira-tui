# Makefile Quick Reference

This project includes a comprehensive Makefile with common development commands.

## Quick Start

```bash
# See all available commands
make help

# Run the application
make run

# Build release binary
make release
```

## Common Commands

### Development
```bash
make run              # Run in debug mode
make build            # Build in debug mode
make dev              # Run with auto-reload (requires cargo-watch)
make check            # Quick check without building
```

### Testing
```bash
make test             # Run all tests
make test-verbose     # Run tests with output
make bench            # Run benchmarks
```

### Code Quality
```bash
make fmt              # Format code
make lint             # Run clippy
make pre-commit       # Run all checks (fmt + lint + test)
make ci               # Run CI checks locally
```

### Release
```bash
make release          # Build optimized binary
make install          # Install to ~/.cargo/bin
make size             # Show binary size
make release-check    # Check if ready for release
```

### Utilities
```bash
make clean            # Remove build artifacts
make doc              # Generate and open documentation
make deps             # List dependencies
make lines            # Count lines of code
make todo             # Show TODO comments
```

### Release Management
```bash
# Create a release tag
make tag VERSION=0.1.0-beta.1

# This will:
# 1. Create an annotated git tag
# 2. Show command to push the tag
```

## Workflow Examples

### Before Committing
```bash
make pre-commit
# This runs: fmt + lint + test
```

### Creating a Release
```bash
# 1. Check everything is ready
make release-check

# 2. Create tag
make tag VERSION=0.2.0

# 3. Push tag
git push origin v0.2.0
```

### Daily Development
```bash
# Terminal 1: Auto-reload on changes
make dev

# Terminal 2: Watch and check
make watch
```

## Optional Tools

Some commands require additional cargo tools:

```bash
# For auto-reload
cargo install cargo-watch

# For dependency updates
cargo install cargo-outdated

# For security audits
cargo install cargo-audit

# For binary size analysis
cargo install cargo-bloat

# For profiling
cargo install flamegraph
```

## Tips

- Run `make help` anytime to see all available commands
- Colors help identify command categories
- All commands handle errors gracefully
- The Makefile is self-documenting

## Customization

You can customize the Makefile by editing these variables at the top:

```makefile
BINARY_NAME := jira-tui
CARGO := cargo
RUST_LOG ?= info
```
