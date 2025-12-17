# Release Notes - v0.1.0-beta.1

## 🎉 First Beta Release

This is the first beta release of **Jira TUI** - a terminal user interface for managing Jira issues and worklogs.

## ✨ Features

### Core Functionality
- **Board Navigation**: Browse and select Jira boards
- **Issue Management**: View issues with detailed information
- **Advanced Filtering**:
  - Filter by assignee (Me, Unassigned, All)
  - Filter by status (To Do, In Progress, Done, All)
  - Sort by updated or created date
- **Infinite Scrolling**: Auto-load more issues as you scroll

### Worklog Management (Time Tracking)
- **Create**: Add worklogs with custom date/time
- **Read**: View all worklogs for an issue in a paginated table
- **Update**: Edit existing worklogs
- **Delete**: Remove worklogs

### User Experience
- **Adaptive UI**: Tables that adjust to terminal size
- **Vim-like Navigation**: Use `j/k` or arrow keys
- **Visual Feedback**: Success/error notifications with emojis
- **Responsive**: Async operations don't block the UI

## 🏗️ Architecture

Built with:
- **Hexagonal Architecture** (Clean Architecture)
- **Rust** with async/await (Tokio)
- **Ratatui** for TUI
- **Strict layer separation** for maintainability

## 📦 Installation

### From Binary

Download from [Releases](https://github.com/your-username/jira-tui/releases):

```bash
# Linux x86_64
wget https://github.com/your-username/jira-tui/releases/download/v0.1.0-beta.1/jira-tui-linux-x86_64
chmod +x jira-tui-linux-x86_64
mv jira-tui-linux-x86_64 jira-tui

# macOS Intel
wget https://github.com/your-username/jira-tui/releases/download/v0.1.0-beta.1/jira-tui-macos-x86_64
chmod +x jira-tui-macos-x86_64
mv jira-tui-macos-x86_64 jira-tui

# macOS Apple Silicon
wget https://github.com/your-username/jira-tui/releases/download/v0.1.0-beta.1/jira-tui-macos-aarch64
chmod +x jira-tui-macos-aarch64
mv jira-tui-macos-aarch64 jira-tui
```

### From Source

```bash
git clone https://github.com/your-username/jira-tui.git
cd jira-tui
cargo build --release
# Binary at target/release/jira-tui
```

## ⚙️ Configuration

Create a `.env` file:

```env
JIRA_BASE_URL=https://your-domain.atlassian.net
JIRA_EMAIL=your-email@example.com
JIRA_API_TOKEN=your_api_token
```

Get your API token: https://id.atlassian.com/manage-profile/security/api-tokens

## 🐛 Known Issues

- Status transitions not yet implemented
- Comments feature pending
- Only supports single project at a time
- No JQL search UI (uses default filters)

## 📝 Roadmap

See [README.md](README.md#roadmap) for upcoming features.

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

CC BY-NC-SA 4.0 - Non-commercial use only. See [LICENSE](LICENSE) for details.

---

## 📊 Build Information

- **Binary size**: ~4.5 MB (stripped)
- **Rust version**: 1.70+
- **Optimizations**: LTO enabled, stripped symbols
- **Platforms**: Linux (x86_64, musl), macOS (x86_64, aarch64)

## 🙏 Credits

Built with [Ratatui](https://github.com/ratatui-org/ratatui) and ❤️ by the Rust community.
