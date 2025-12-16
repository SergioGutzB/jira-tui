use crate::domain::models::{Board, Issue};

/// Represents the current active screen/view in the TUI.
#[derive(Debug, Clone, PartialEq)]
pub enum CurrentScreen {
    /// The initial loading or dashboard screen.
    Dashboard,
    /// List of available Jira boards.
    BoardsList,
    /// Backlog/Issues list for a selected board.
    Backlog,
    /// Detailed view of a specific issue.
    IssueDetail,
    /// An input prompt or modal (e.g., for comments).
    Exiting,
}

/// Represents an intent or command triggered by the user or system.
#[derive(Debug, Clone)]
pub enum Action {
    /// Application tick (UI update).
    Tick,
    /// Quit the application.
    Quit,
    /// Resize the terminal.
    Resize(u16, u16),
    /// Navigate to the Boards list.
    GoToBoards,
    /// Navigate to the Backlog.
    GoToBacklog,
    /// Move selection down.
    SelectNext,
    /// Move selection up.
    SelectPrevious,
    /// Confirm selection (Enter key).
    Enter,
    // Data Loading Actions
    /// Request to load boards from the API.
    LoadBoards,
    /// Callback when boards are successfully loaded.
    BoardsLoaded(Vec<Board>),
}

/// Holds the global state of the TUI application.
pub struct App {
    /// Indicates if the application should terminate.
    pub should_quit: bool,
    /// The currently active view.
    pub current_screen: CurrentScreen,

    // --- Data State ---
    pub boards: Vec<Board>,
    pub selected_board_index: usize,
    pub issues: Vec<Issue>,
    pub selected_issue_index: usize,
    pub is_loading: bool,
}

impl App {
    /// Initializes a new application state with default values.
    pub fn new() -> Self {
        Self {
            should_quit: false,
            current_screen: CurrentScreen::Dashboard,
            boards: Vec::new(),
            selected_board_index: 0,
            issues: Vec::new(),
            selected_issue_index: 0,
            is_loading: false,
        }
    }

    /// Updates the application state based on an action.
    pub fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::Tick => {} // Handle animations if needed
            Action::Resize(_, _) => {}

            Action::GoToBoards => {
                self.current_screen = CurrentScreen::BoardsList;
            }

            Action::BoardsLoaded(boards) => {
                self.boards = boards;
                self.is_loading = false;
                self.current_screen = CurrentScreen::BoardsList;
            }

            Action::SelectNext => match self.current_screen {
                CurrentScreen::BoardsList => {
                    if !self.boards.is_empty() {
                        let next = self.selected_board_index.saturating_add(1);
                        self.selected_board_index =
                            if next >= self.boards.len() { 0 } else { next };
                    }
                }
                CurrentScreen::Backlog => {
                    if !self.issues.is_empty() {
                        let next = self.selected_issue_index.saturating_add(1);
                        self.selected_issue_index =
                            if next >= self.issues.len() { 0 } else { next };
                    }
                }
                _ => {}
            },

            Action::SelectPrevious => match self.current_screen {
                CurrentScreen::BoardsList => {
                    if !self.boards.is_empty() {
                        self.selected_board_index = if self.selected_board_index == 0 {
                            self.boards.len() - 1
                        } else {
                            self.selected_board_index - 1
                        };
                    }
                }
                CurrentScreen::Backlog => {
                    if !self.issues.is_empty() {
                        self.selected_issue_index = if self.selected_issue_index == 0 {
                            self.issues.len() - 1
                        } else {
                            self.selected_issue_index - 1
                        };
                    }
                }
                _ => {}
            },

            _ => {} // Other actions to be implemented
        }
    }
}
