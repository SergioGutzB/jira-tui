use crate::domain::models::{Board, Issue, Paginated};

#[derive(Debug, Clone, PartialEq)]
pub enum CurrentScreen {
    Dashboard,
    BoardsList,
    Backlog,
    IssueDetail,
    Exiting,
}

#[derive(Debug, Clone)]
pub enum Action {
    Tick,
    Quit,
    Resize(u16, u16),
    GoToBoards,
    GoToBacklog,

    SelectNext,
    SelectPrevious,
    Enter,
    ViewIssueDetail,

    LoadBoards,
    BoardsLoaded(Vec<Board>),
    LoadIssues(u64),
    IssuesLoaded(Paginated<Issue>),
    LoadMoreIssues,
}

pub struct App {
    pub should_quit: bool,
    pub current_screen: CurrentScreen,

    pub boards: Vec<Board>,
    pub selected_board_index: usize,

    pub issues: Vec<Issue>,
    pub selected_issue_index: usize,

    pub is_loading: bool,

    pub vertical_scroll: u16,
    pub total_issues: u64,
    pub current_board_id: Option<u64>,
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
            vertical_scroll: 0,
            total_issues: 0,
            current_board_id: None,
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

            Action::ViewIssueDetail => {
                if !self.issues.is_empty() {
                    self.current_screen = CurrentScreen::IssueDetail;
                    self.vertical_scroll = 0;
                }
            }

            Action::LoadIssues(board_id) => {
                self.is_loading = true;
                self.current_screen = CurrentScreen::Backlog;
                self.issues.clear(); // Limpiamos para nueva búsqueda
                self.vertical_scroll = 0;
                self.current_board_id = Some(board_id);
                self.total_issues = 0;
            }

            Action::IssuesLoaded(paginated) => {
                self.is_loading = false;
                self.current_screen = CurrentScreen::Backlog;

                if paginated.start_at == 0 {
                    self.issues = paginated.items;
                    self.selected_issue_index = 0;
                } else {
                    self.issues.extend(paginated.items);
                }

                self.total_issues = paginated.total;
            }

            Action::SelectNext => match self.current_screen {
                CurrentScreen::BoardsList | CurrentScreen::Dashboard => {
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
                CurrentScreen::IssueDetail => {
                    self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                }
                _ => {}
            },

            Action::SelectPrevious => match self.current_screen {
                CurrentScreen::BoardsList | CurrentScreen::Dashboard => {
                    if self.selected_board_index > 0 {
                        self.selected_board_index -= 1;
                    }
                }
                CurrentScreen::Backlog => {
                    if self.selected_issue_index > 0 {
                        self.selected_issue_index -= 1;
                    }
                }
                CurrentScreen::IssueDetail => {
                    if self.vertical_scroll > 0 {
                        self.vertical_scroll -= 1;
                    }
                }
                _ => {}
            },

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
