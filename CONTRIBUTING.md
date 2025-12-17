# Contributing Guide

Thank you for your interest in contributing to Jira TUI! This document will guide you through the contribution process.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Pull Request Process](#pull-request-process)
- [Code Standards](#code-standards)
- [Commit Conventions](#commit-conventions)
- [Environment Setup](#environment-setup)

## Code of Conduct

This project adheres to a professional and respectful code of conduct:

- **Be respectful**: Treat everyone with respect and consideration
- **Be constructive**: Criticism should be constructive and focused on improving the code
- **Be collaborative**: We work together to create something better
- **Be patient**: We're all learning

## How Can I Contribute?

### 🐛 Reporting Bugs

If you find a bug, please create an issue with:

1. **Descriptive title**: Brief description of the problem
2. **Steps to reproduce**: How you encountered the bug
3. **Expected behavior**: What you expected to happen
4. **Actual behavior**: What actually happened
5. **Environment**:
   - OS and version
   - Rust version
   - Application version
6. **Logs** (if possible): Run with `RUST_LOG=debug`

### ✨ Proposing Features

To propose a new feature:

1. **Check existing issues**: Make sure it hasn't been proposed already
2. **Create an issue** with:
   - Clear description of the feature
   - Use cases
   - Mockups or examples (if applicable)
   - Why it would add value to the project

### 💻 Contributing Code

1. **Fork** the repository
2. **Clone** your fork locally
3. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/my-feature
   # or
   git checkout -b fix/my-fix
   ```
4. **Implement** your changes
5. **Test** your changes
6. **Commit** following the [conventions](#commit-conventions)
7. **Push** to your fork
8. **Create** a Pull Request

## Pull Request Process

### Before Submitting

- [ ] Code compiles without errors (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] Clippy generates no warnings (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] You've updated documentation if necessary
- [ ] You've added tests for new code

### Creating the PR

1. **Clear title**: Describe what the PR does in one line
2. **Detailed description**:
   ```markdown
   ## Description
   Brief description of changes

   ## Type of change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation

   ## How has this been tested?
   Describe the tests you performed

   ## Screenshots (if applicable)
   Add TUI screenshots

   ## Checklist
   - [ ] My code follows the project conventions
   - [ ] I have performed a self-review of my code
   - [ ] I have commented my code in hard-to-understand areas
   - [ ] I have updated the documentation
   - [ ] My changes generate no new warnings
   - [ ] I have added tests that prove my fix/feature
   - [ ] New and existing unit tests pass
   ```

### Review Process

1. **Maintainer will review** your PR within 7 days
2. **Changes will be requested** if necessary
3. **Approval**: Once approved, it will be merged to `main`
4. **Release**: Changes will be included in the next release

### What to Expect

- **Constructive feedback**: There may be suggestions for improvement
- **Requested changes**: It's normal to be asked for adjustments
- **Response time**: Please be patient, this is a community-maintained project

## Code Standards

### Architecture

The project follows **Hexagonal Architecture**. Respect the layers:

```
domain/ → No external dependencies, only business logic
application/ → Use cases, orchestrates domain
infrastructure/ → Adapters (API, DB, etc.)
ui/ → Presentation layer (TUI)
```

**Rules**:
- ❌ Domain CANNOT import from application, infrastructure or ui
- ❌ Application CANNOT import from infrastructure or ui
- ✅ Infrastructure CAN import from domain
- ✅ UI CAN import from application and domain

### Code Style

#### Naming

- **ALWAYS snake_case**: Variables, functions, struct fields, modules
```rust
// ✅ Correct
let user_name = "John";
fn get_user_data() {}
pub struct UserData { first_name: String }

// ❌ Incorrect
let userName = "John";
fn getUserData() {}
pub struct UserData { firstName: String }
```

- **PascalCase**: Types, Traits, Enums
```rust
struct UserAccount {}
trait Repository {}
enum Status {}
```

#### Doc Comments

```rust
/// Retrieves user data from the repository
///
/// # Arguments
/// * `user_id` - The unique identifier
///
/// # Returns
/// A Result containing UserData or an error
pub fn get_user(user_id: u64) -> Result<UserData> {
    // ...
}
```

#### Error Handling

- Use `Result<T, E>` for operations that can fail
- Use `thiserror` for library/domain errors
- Use `anyhow` for application errors
- DO NOT use `unwrap()` or `expect()` in production code

```rust
// ✅ Correct
let data = repository.get_data()
    .map_err(|e| AppError::DataNotFound(e.to_string()))?;

// ❌ Avoid
let data = repository.get_data().unwrap();
```

#### Imports

Organize imports in groups:

```rust
// 1. Standard library
use std::sync::Arc;

// 2. External crates
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};

// 3. Crate modules
use crate::domain::models::User;
use crate::application::use_cases::GetUserUseCase;
```

### Testing

- Write unit tests for business logic
- Use `#[cfg(test)]` for test modules
- Name test functions descriptively

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_with_valid_data_succeeds() {
        // Arrange
        let name = "John";

        // Act
        let user = User::new(name);

        // Assert
        assert_eq!(user.name, name);
    }
}
```

## Commit Conventions

We use **Conventional Commits**:

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting, semicolons, etc (doesn't affect code)
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `test`: Add or fix tests
- `chore`: Maintenance tasks

### Examples

```bash
feat(ui): add worklog list modal with pagination

fix(client): handle 404 errors correctly in worklog fetch

docs(readme): update installation instructions

refactor(handlers): extract worklog logic to separate functions

test(models): add tests for WorklogEntry validation
```

## Environment Setup

### Requirements

- Rust 1.70+
- Cargo
- Git

### Setup

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/jira-tui.git
cd jira-tui

# 2. Configure upstream
git remote add upstream https://github.com/ORIGINAL_AUTHOR/jira-tui.git

# 3. Install tools
rustup component add clippy rustfmt

# 4. Configure .env
cp .env.example .env
# Edit .env with your Jira credentials

# 5. Build
cargo build

# 6. Run tests
cargo test

# 7. Run
cargo run
```

### Recommended Tools

- **Editor**: VSCode with rust-analyzer, Vim/Neovim with rust-analyzer
- **Debugger**: lldb or gdb
- **Git GUI**: GitKraken, GitHub Desktop, or command line

### Development Workflow

```bash
# 1. Update your main
git checkout main
git pull upstream main

# 2. Create feature branch
git checkout -b feature/my-feature

# 3. Develop
# ... code ...

# 4. Check quality
cargo fmt
cargo clippy
cargo test

# 5. Commit
git add .
git commit -m "feat(ui): add new feature"

# 6. Push to your fork
git push origin feature/my-feature

# 7. Create PR on GitHub
```

## Frequently Asked Questions

### How long does it take to review a PR?

Generally within 7 days. If it's urgent, mention the maintainer in the PR.

### Can I work on multiple issues?

Yes, but we recommend focusing on one at a time to avoid merge conflicts.

### What if my PR is rejected?

You'll receive feedback explaining why. You can make adjustments and resubmit.

### Do I need permission to work on an issue?

Not necessary, but comment on the issue that you'll work on it to avoid duplication.

### Can I contribute without knowing Rust?

Yes! You can contribute documentation, report bugs, propose features, etc.

## Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Ratatui Tutorial](https://ratatui.rs/tutorials/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

---

## 🙏 Thanks

Thank you for contributing to Jira TUI. Your time and effort are greatly appreciated.

If you have questions not answered here, feel free to open a discussion issue.

**Happy Coding! 🦀**
