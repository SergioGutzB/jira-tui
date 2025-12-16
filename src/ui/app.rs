use crate::domain::models::{Board, Issue};

#[derive(Debug, Clone, PartialEq)]
pub enum CurrentScreen {
    Dashboard,
    BoardsList,
    Backlog,
    IssueDetail, // View for specific issue info
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
    Enter,

    // Data Loading
    LoadBoards,
    BoardsLoaded(Vec<Board>),
    LoadIssues(u64),
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

    // New: State for text scrolling in Detail view
    pub vertical_scroll: u16,
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
            vertical_scroll: 0, // Init
        }
    }

    pub fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::Tick => {}
            Action::Resize(_, _) => {}

            Action::GoToBoards => {
                self.current_screen = CurrentScreen::BoardsList;
                self.vertical_scroll = 0;
            }

            Action::BoardsLoaded(boards) => {
                self.boards = boards;
                self.selected_board_index = 0;
                self.is_loading = false;
                self.current_screen = CurrentScreen::BoardsList;
                self.vertical_scroll = 0;
            }

            Action::LoadIssues(_) => {
                self.is_loading = true;
                self.current_screen = CurrentScreen::Backlog;
                self.issues.clear();
                self.vertical_scroll = 0;
            }

            Action::IssuesLoaded(issues) => {
                self.issues = issues;
                self.selected_issue_index = 0;
                self.is_loading = false;
                self.current_screen = CurrentScreen::Backlog;
                self.vertical_scroll = 0;
            }

            Action::SelectNext => {
                match self.current_screen {
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
                    // Scroll Down in Detail View
                    CurrentScreen::IssueDetail => {
                        self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                    }
                    _ => {}
                }
            }

            Action::SelectPrevious => {
                match self.current_screen {
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
                    // Scroll Up in Detail View
                    CurrentScreen::IssueDetail => {
                        if self.vertical_scroll > 0 {
                            self.vertical_scroll -= 1;
                        }
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }

    pub fn get_selected_board(&self) -> Option<&Board> {
        self.boards.get(self.selected_board_index)
    }

    pub fn get_selected_issue(&self) -> Option<&Issue> {
        self.issues.get(self.selected_issue_index)
    }
}
