use crate::domain::models::{Board, Issue};

#[derive(Debug, Clone, PartialEq)]
pub enum CurrentScreen {
    Dashboard,
    BoardsList,
    Backlog, // We will use this now
    IssueDetail,
    Exiting,
}

#[derive(Debug, Clone)]
pub enum Action {
    Tick,
    Quit,
    Resize(u16, u16),

    // Navigation
    GoToBoards,
    GoToBacklog,
    SelectNext,
    SelectPrevious,
    Enter, // Generic "Confirm" action

    // Data Loading
    LoadBoards,
    BoardsLoaded(Vec<Board>),

    // New Actions for Issues
    LoadIssues(u64), // Carries the Board ID
    IssuesLoaded(Vec<Issue>),
}

pub struct App {
    pub should_quit: bool,
    pub current_screen: CurrentScreen,

    pub boards: Vec<Board>,
    pub selected_board_index: usize,

    pub issues: Vec<Issue>,
    pub selected_issue_index: usize,

    pub is_loading: bool,
}

impl App {
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

    pub fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::Tick => {}
            Action::Resize(_, _) => {}

            Action::GoToBoards => self.current_screen = CurrentScreen::BoardsList,

            Action::BoardsLoaded(boards) => {
                self.boards = boards;
                self.selected_board_index = 0;
                self.is_loading = false;
                self.current_screen = CurrentScreen::BoardsList;
            }

            Action::LoadIssues(_) => {
                self.is_loading = true;
                // We switch screen immediately to show "Loading" context
                self.current_screen = CurrentScreen::Backlog;
                // Clear previous issues to avoid showing stale data
                self.issues.clear();
            }

            Action::IssuesLoaded(issues) => {
                self.issues = issues;
                self.selected_issue_index = 0;
                self.is_loading = false;
                // Ensure we remain on Backlog screen
                self.current_screen = CurrentScreen::Backlog;
            }

            Action::SelectNext => match self.current_screen {
                CurrentScreen::BoardsList => {
                    if !self.boards.is_empty() {
                        let next = self.selected_board_index.saturating_add(1);
                        if next < self.boards.len() {
                            self.selected_board_index = next;
                        }
                    }
                }
                CurrentScreen::Backlog => {
                    if !self.issues.is_empty() {
                        let next = self.selected_issue_index.saturating_add(1);
                        if next < self.issues.len() {
                            self.selected_issue_index = next;
                        }
                    }
                }
                _ => {}
            },

            Action::SelectPrevious => match self.current_screen {
                CurrentScreen::BoardsList => {
                    if self.selected_board_index > 0 {
                        self.selected_board_index -= 1;
                    }
                }
                CurrentScreen::Backlog => {
                    if self.selected_issue_index > 0 {
                        self.selected_issue_index -= 1;
                    }
                }
                _ => {}
            },

            _ => {}
        }
    }

    /// Helper to retrieve the currently selected board.
    pub fn get_selected_board(&self) -> Option<&Board> {
        self.boards.get(self.selected_board_index)
    }
}
