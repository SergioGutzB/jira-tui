use crate::domain::models::{AssigneeFilter, Board, Issue, OrderByFilter, Paginated, StatusFilter};
use chrono::{Local, Datelike, Timelike};

#[derive(Debug, Clone, PartialEq)]
pub enum CurrentScreen {
    Dashboard,
    BoardsList,
    Backlog,
    IssueDetail,
    FilterModal,
    WorklogModal,
    Exiting,
}

/// Represents which field is currently focused in the filter modal
#[derive(Debug, Clone, PartialEq)]
pub enum FilterField {
    Assignee,
    Status,
    OrderBy,
}

/// Represents which field is currently focused in the worklog modal
#[derive(Debug, Clone, PartialEq)]
pub enum WorklogField {
    Day,
    Month,
    Year,
    Hour,
    Minute,
    TimeHours,
    TimeMinutes,
    Comment,
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

    OpenFilterModal,
    CloseFilterModal,
    NextFilterField,
    CycleAssigneeFilter,
    CycleStatusFilter,
    CycleOrderByFilter,
    ApplyFilter,

    OpenWorklogModal,
    CloseWorklogModal,
    NextWorklogField,
    InputWorklogDigit(char),
    InputWorklogChar(char),
    DeleteWorklogChar,
    SubmitWorklog,
    WorklogSubmitted,

    ShowNotification(String, String, bool),
    HideNotification,
}

pub struct App {
    pub should_quit: bool,
    pub current_screen: CurrentScreen,
    pub previous_screen: Option<CurrentScreen>,

    pub boards: Vec<Board>,
    pub selected_board_index: usize,

    pub issues: Vec<Issue>,
    pub selected_issue_index: usize,

    pub is_loading: bool,

    pub vertical_scroll: u16,
    pub total_issues: u64,
    pub current_board_id: Option<u64>,

    pub filter_assignee: AssigneeFilter,
    pub filter_status: StatusFilter,
    pub filter_order_by: OrderByFilter,
    pub filter_focused_field: FilterField,

    pub worklog_day: u8,
    pub worklog_month: u8,
    pub worklog_year: u16,
    pub worklog_hour: u8,
    pub worklog_minute: u8,
    pub worklog_time_hours: u8,
    pub worklog_time_minutes: u8,
    pub worklog_comment: String,
    pub worklog_focused_field: WorklogField,

    pub notification_title: Option<String>,
    pub notification_message: Option<String>,
    pub notification_is_success: bool,
}

impl App {
    pub fn new() -> Self {
        let now = Local::now();
        Self {
            should_quit: false,
            current_screen: CurrentScreen::Dashboard,
            previous_screen: None,
            boards: Vec::new(),
            selected_board_index: 0,
            issues: Vec::new(),
            selected_issue_index: 0,
            is_loading: false,
            vertical_scroll: 0,
            total_issues: 0,
            current_board_id: None,
            filter_assignee: AssigneeFilter::CurrentUser,
            filter_status: StatusFilter::All,
            filter_order_by: OrderByFilter::UpdatedDesc,
            filter_focused_field: FilterField::Assignee,
            worklog_day: now.day() as u8,
            worklog_month: now.month() as u8,
            worklog_year: now.year() as u16,
            worklog_hour: now.hour() as u8,
            worklog_minute: now.minute() as u8,
            worklog_time_hours: 0,
            worklog_time_minutes: 0,
            worklog_comment: String::new(),
            worklog_focused_field: WorklogField::Day,
            notification_title: None,
            notification_message: None,
            notification_is_success: false,
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

            Action::GoToBacklog => {
                self.current_screen = CurrentScreen::Backlog;
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

            Action::OpenFilterModal => {
                self.previous_screen = Some(self.current_screen.clone());
                self.current_screen = CurrentScreen::FilterModal;
                self.filter_focused_field = FilterField::Assignee;
            }

            Action::CloseFilterModal => {
                if let Some(prev) = self.previous_screen.take() {
                    self.current_screen = prev;
                } else {
                    self.current_screen = CurrentScreen::Backlog;
                }
            }

            Action::NextFilterField => {
                self.filter_focused_field = match self.filter_focused_field {
                    FilterField::Assignee => FilterField::Status,
                    FilterField::Status => FilterField::OrderBy,
                    FilterField::OrderBy => FilterField::Assignee,
                };
            }

            Action::CycleAssigneeFilter => {
                self.filter_assignee = match self.filter_assignee {
                    AssigneeFilter::CurrentUser => AssigneeFilter::Unassigned,
                    AssigneeFilter::Unassigned => AssigneeFilter::All,
                    AssigneeFilter::All => AssigneeFilter::CurrentUser,
                };
            }

            Action::CycleStatusFilter => {
                self.filter_status = match self.filter_status {
                    StatusFilter::All => StatusFilter::Todo,
                    StatusFilter::Todo => StatusFilter::InProgress,
                    StatusFilter::InProgress => StatusFilter::Done,
                    StatusFilter::Done => StatusFilter::All,
                };
            }

            Action::CycleOrderByFilter => {
                self.filter_order_by = match self.filter_order_by {
                    OrderByFilter::UpdatedDesc => OrderByFilter::CreatedDesc,
                    OrderByFilter::CreatedDesc => OrderByFilter::UpdatedDesc,
                };
            }

            Action::OpenWorklogModal => {
                let now = Local::now();
                self.previous_screen = Some(self.current_screen.clone());
                self.current_screen = CurrentScreen::WorklogModal;
                self.worklog_day = now.day() as u8;
                self.worklog_month = now.month() as u8;
                self.worklog_year = now.year() as u16;
                self.worklog_hour = now.hour() as u8;
                self.worklog_minute = now.minute() as u8;
                self.worklog_time_hours = 0;
                self.worklog_time_minutes = 0;
                self.worklog_comment.clear();
                self.worklog_focused_field = WorklogField::Day;
            }

            Action::CloseWorklogModal => {
                if let Some(prev) = self.previous_screen.take() {
                    self.current_screen = prev;
                } else {
                    self.current_screen = CurrentScreen::IssueDetail;
                }
            }

            Action::NextWorklogField => {
                self.worklog_focused_field = match self.worklog_focused_field {
                    WorklogField::Day => WorklogField::Month,
                    WorklogField::Month => WorklogField::Year,
                    WorklogField::Year => WorklogField::Hour,
                    WorklogField::Hour => WorklogField::Minute,
                    WorklogField::Minute => WorklogField::TimeHours,
                    WorklogField::TimeHours => WorklogField::TimeMinutes,
                    WorklogField::TimeMinutes => WorklogField::Comment,
                    WorklogField::Comment => WorklogField::Day,
                };
            }

            Action::InputWorklogDigit(digit) => {
                let digit_val = digit.to_digit(10).unwrap_or(0);
                match self.worklog_focused_field {
                    WorklogField::Day => {
                        let new_value = (self.worklog_day as u16) * 10 + digit_val as u16;
                        self.worklog_day = if new_value > 31 { digit_val as u8 } else { new_value as u8 };
                    }
                    WorklogField::Month => {
                        let new_value = (self.worklog_month as u16) * 10 + digit_val as u16;
                        self.worklog_month = if new_value > 12 { digit_val as u8 } else { new_value as u8 };
                    }
                    WorklogField::Year => {
                        let new_value = (self.worklog_year as u32) * 10 + digit_val as u32;
                        self.worklog_year = if new_value > 9999 { digit_val as u16 } else { new_value as u16 };
                    }
                    WorklogField::Hour => {
                        let new_value = (self.worklog_hour as u16) * 10 + digit_val as u16;
                        self.worklog_hour = if new_value > 23 { digit_val as u8 } else { new_value as u8 };
                    }
                    WorklogField::Minute => {
                        let new_value = (self.worklog_minute as u16) * 10 + digit_val as u16;
                        self.worklog_minute = if new_value > 59 { digit_val as u8 } else { new_value as u8 };
                    }
                    WorklogField::TimeHours => {
                        let new_value = (self.worklog_time_hours as u16) * 10 + digit_val as u16;
                        self.worklog_time_hours = if new_value > 99 { digit_val as u8 } else { new_value as u8 };
                    }
                    WorklogField::TimeMinutes => {
                        let new_value = (self.worklog_time_minutes as u16) * 10 + digit_val as u16;
                        self.worklog_time_minutes = if new_value > 59 { digit_val as u8 } else { new_value as u8 };
                    }
                    WorklogField::Comment => {
                        self.worklog_comment.push(digit);
                    }
                }
            }

            Action::InputWorklogChar(ch) => {
                if matches!(self.worklog_focused_field, WorklogField::Comment) {
                    self.worklog_comment.push(ch);
                }
            }

            Action::DeleteWorklogChar => {
                match self.worklog_focused_field {
                    WorklogField::Day => self.worklog_day /= 10,
                    WorklogField::Month => self.worklog_month /= 10,
                    WorklogField::Year => self.worklog_year /= 10,
                    WorklogField::Hour => self.worklog_hour /= 10,
                    WorklogField::Minute => self.worklog_minute /= 10,
                    WorklogField::TimeHours => self.worklog_time_hours /= 10,
                    WorklogField::TimeMinutes => self.worklog_time_minutes /= 10,
                    WorklogField::Comment => {
                        self.worklog_comment.pop();
                    }
                }
            }

            Action::WorklogSubmitted => {
                let now = Local::now();
                self.worklog_day = now.day() as u8;
                self.worklog_month = now.month() as u8;
                self.worklog_year = now.year() as u16;
                self.worklog_hour = now.hour() as u8;
                self.worklog_minute = now.minute() as u8;
                self.worklog_time_hours = 0;
                self.worklog_time_minutes = 0;
                self.worklog_comment.clear();
                if let Some(prev) = self.previous_screen.take() {
                    self.current_screen = prev;
                }
            }

            Action::ShowNotification(title, message, is_success) => {
                self.notification_title = Some(title);
                self.notification_message = Some(message);
                self.notification_is_success = is_success;
            }

            Action::HideNotification => {
                self.notification_title = None;
                self.notification_message = None;
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
