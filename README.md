# Jira TUI

> Terminal User Interface for Jira - Manage your issues and worklogs from the terminal

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Version](https://img.shields.io/badge/version-0.1.0--beta-blue?style=for-the-badge)
![License](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg?style=for-the-badge)

## 🎬 Demo

### Filtering Issues

Filter your issues by assignee, status, and sort order with an intuitive modal interface:

![Filter Demo](docs/demo_filter.gif)

### Managing Worklogs

Easily add, edit, and manage your time tracking directly from the terminal:

![Worklog Demo](docs/demo_worklog.gif)

## 📋 Description

**Jira TUI** is a terminal application that allows you to interact with Jira efficiently without leaving your command line. Built with Rust following Hexagonal Architecture (Clean Architecture) principles.

### ✨ Features

- 📊 **Board Navigation**: List and select your Jira boards
- 📝 **Issue Management**: View issues with customizable filters
- ⏱️ **Worklogs (Time Tracking)**:
  - Add work time with customizable date/time
  - List all worklogs for an issue
  - Edit existing worklogs
  - Delete worklogs
- 🔍 **Advanced Filters**:
  - By assignee (Me, Unassigned, All)
  - By status (To Do, In Progress, Done, All)
  - Sort by (Recently Updated, Recently Created)
- 📄 **Infinite Pagination**: Auto-scroll to load more issues
- 🎨 **Adaptive UI**: Tables with columns that adjust to terminal size
- ✅ **Notifications**: Visual feedback with emojis for successful/failed operations

## 🚀 Installation

### Prerequisites

- Rust 1.70 or higher
- Jira account with API access
- Jira API Token ([Create token](https://id.atlassian.com/manage-profile/security/api-tokens))

### From source

```bash
# Clone the repository
git clone https://github.com/your-username/jira-tui.git
cd jira-tui

# Build in release mode
cargo build --release

# Binary will be at target/release/jira-tui
```

### From release

Download the compiled binary for your platform from [Releases](https://github.com/your-username/jira-tui/releases).

## ⚙️ Configuration

Create a `.env` file in the project root directory:

```bash
cp .env.example .env
```

Edit the `.env` file with your credentials:

```env
JIRA_BASE_URL=https://your-domain.atlassian.net
JIRA_EMAIL=your-email@example.com
JIRA_API_TOKEN=your_api_token
```

### Getting a Jira API Token

1. Go to [Atlassian API Tokens](https://id.atlassian.com/manage-profile/security/api-tokens)
2. Create a new token
3. Copy the generated token to your `.env` file

## 🎮 Usage

```bash
# Run the application
cargo run

# Or if you installed the binary
./jira-tui
```

### Navigation

#### Global
- `q` - Quit application
- `Esc` - Go back to previous screen

#### Boards List
- `b` - Load boards from Jira
- `j/k` or `↓/↑` - Navigate list
- `Enter` - Select board and load issues

#### Backlog (Issues List)
- `j/k` or `↓/↑` - Navigate list
- `Enter` - View issue details
- `f` - Open filters modal
- `b` or `Esc` - Back to boards

#### Issue Detail
- `j/k` or `↓/↑` - Scroll content
- `w` - Add new worklog
- `l` - List issue worklogs
- `Esc` - Back to backlog

#### Filters Modal
- `Tab` or `j/k` - Switch between fields
- `h/l` or `←/→` - Change filter value
- `Enter` - Apply filters
- `Esc` - Cancel

#### Worklog Modal (Add/Edit)
- `Tab` or `j/k` - Switch between fields
- `0-9` - Enter numbers (date, time, duration)
- `a-z, space` - Enter text (comment)
- `Backspace` - Delete last character
- `Enter` - Save worklog
- `Esc` - Cancel

#### Worklog List Modal
- `j/k` or `↓/↑` - Navigate list
- `Enter` or `e` - Edit selected worklog
- `d` - Delete selected worklog
- `Esc` - Close modal

## 🏗️ Architecture

The project follows **Hexagonal Architecture (Ports & Adapters)**:

```
src/
├── domain/               # Business core (no external dependencies)
│   ├── models.rs         # Domain entities
│   ├── repositories.rs   # Traits (Ports)
│   └── errors.rs         # Domain errors
├── application/          # Use cases
│   └── use_cases.rs      # Application logic
├── infrastructure/       # External adapters
│   ├── config.rs         # Configuration
│   └── jira/             # Jira API implementation
│       ├── client.rs     # HTTP client
│       └── dtos.rs       # API DTOs
└── ui/                   # Presentation layer (TUI)
    ├── app.rs            # Application state
    ├── events.rs         # Event handling
    ├── handlers.rs       # Async side effects
    ├── keys.rs           # Key mapping
    └── widgets/          # UI components
```

### Main Technologies

- **[Ratatui](https://github.com/ratatui-org/ratatui)**: TUI framework
- **[Tokio](https://tokio.rs/)**: Async runtime
- **[Reqwest](https://github.com/seanmonstar/reqwest)**: HTTP client
- **[Serde](https://serde.rs/)**: JSON serialization
- **[Chrono](https://github.com/chronotope/chrono)**: Date/time handling

## 🔧 Development

### Run with logs

```bash
RUST_LOG=debug cargo run
```

### Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

### Format

```bash
cargo fmt
```

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) to learn about the contribution process.

### Quick process

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 Roadmap

- [x] Board and issue navigation
- [x] Issue filters
- [x] Complete worklog management (CRUD)
- [x] Visual notifications
- [ ] Issue status transitions
- [ ] Add comments to issues
- [ ] Multi-project support
- [ ] Configuration from `.jira/config.toml` file
- [ ] Advanced JQL search
- [ ] Data export

## 📄 License

This project is licensed under [Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International (CC BY-NC-SA 4.0)](LICENSE).

**License summary:**
- ✅ You can use, modify and share
- ✅ You must give credit to the original author
- ❌ **Commercial use is NOT allowed**
- ✅ You must share under the same license

## 👨‍💻 Author

**Sergio Gutierrez**

## 🙏 Acknowledgments

- [Ratatui](https://github.com/ratatui-org/ratatui) for the excellent TUI framework
- The Rust community for amazing tools
- All contributors who help improve this project

---

**Note**: This project is in beta phase. It may contain bugs or incomplete features.
