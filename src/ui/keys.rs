use crate::ui::app::{Action, App, CurrentScreen, FilterField, WorklogField};
use crossterm::event::{KeyCode, KeyEvent};

/// Maps a physical key event to an application action based on context.
pub fn from_event(key: KeyEvent, app: &App) -> Option<Action> {
    match app.current_screen {
        CurrentScreen::Dashboard | CurrentScreen::BoardsList => match_boards_keys(key, app),
        CurrentScreen::Backlog => match_backlog_keys(key, app),
        CurrentScreen::IssueDetail => match_detail_keys(key),
        CurrentScreen::FilterModal => match_filter_modal_keys(key, app),
        CurrentScreen::WorklogModal => match_worklog_modal_keys(key, app),
        CurrentScreen::WorklogListModal => match_worklog_list_modal_keys(key),
        _ => match_global_keys(key),
    }
}

fn match_global_keys(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        _ => None,
    }
}

fn match_boards_keys(key: KeyEvent, app: &App) -> Option<Action> {
    match key.code {
        // Global Overrides
        KeyCode::Char('q') => Some(Action::Quit),

        // Context Specific
        KeyCode::Char('b') => Some(Action::LoadBoards),
        KeyCode::Enter => app.get_selected_board().map(|b| Action::LoadIssues(b.id)),

        // Navigation
        KeyCode::Down | KeyCode::Char('j') => Some(Action::SelectNext),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::SelectPrevious),

        _ => None,
    }
}

fn match_backlog_keys(key: KeyEvent, _app: &App) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::GoToBoards),
        KeyCode::Char('b') => Some(Action::GoToBoards),
        KeyCode::Char('q') => Some(Action::Quit),

        KeyCode::Enter => Some(Action::ViewIssueDetail),
        KeyCode::Char('f') => Some(Action::OpenFilterModal),

        KeyCode::Down | KeyCode::Char('j') => Some(Action::SelectNext),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::SelectPrevious),

        _ => None,
    }
}

fn match_detail_keys(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::GoToBacklog),
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('w') => Some(Action::OpenWorklogModal),
        KeyCode::Char('l') => Some(Action::OpenWorklogListModal),

        // Scroll
        KeyCode::Down | KeyCode::Char('j') => Some(Action::SelectNext),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::SelectPrevious),

        _ => None,
    }
}

fn match_filter_modal_keys(key: KeyEvent, app: &App) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::CloseFilterModal),
        KeyCode::Char('q') => Some(Action::Quit),

        KeyCode::Enter => Some(Action::ApplyFilter),

        KeyCode::Tab | KeyCode::Down | KeyCode::Char('j') => Some(Action::NextFilterField),
        KeyCode::BackTab | KeyCode::Up | KeyCode::Char('k') => Some(Action::NextFilterField),

        KeyCode::Left | KeyCode::Char('h') | KeyCode::Right | KeyCode::Char('l') => {
            match app.filter_focused_field {
                FilterField::Assignee => Some(Action::CycleAssigneeFilter),
                FilterField::Status => Some(Action::CycleStatusFilter),
                FilterField::OrderBy => Some(Action::CycleOrderByFilter),
            }
        }

        _ => None,
    }
}

fn match_worklog_modal_keys(key: KeyEvent, app: &App) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::CloseWorklogModal),

        KeyCode::Enter => Some(Action::SubmitWorklog),

        KeyCode::Tab | KeyCode::Down => Some(Action::NextWorklogField),
        KeyCode::BackTab | KeyCode::Up => Some(Action::NextWorklogField),

        KeyCode::Char(ch) if ch.is_ascii_digit() => {
            if matches!(
                app.worklog_focused_field,
                WorklogField::Day
                    | WorklogField::Month
                    | WorklogField::Year
                    | WorklogField::Hour
                    | WorklogField::Minute
                    | WorklogField::TimeHours
                    | WorklogField::TimeMinutes
            ) {
                Some(Action::InputWorklogDigit(ch))
            } else {
                Some(Action::InputWorklogChar(ch))
            }
        }

        KeyCode::Char(ch) if ch.is_alphanumeric() || ch == ' ' => {
            Some(Action::InputWorklogChar(ch))
        }

        KeyCode::Backspace => Some(Action::DeleteWorklogChar),

        _ => None,
    }
}

fn match_worklog_list_modal_keys(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::CloseWorklogListModal),
        KeyCode::Char('q') => Some(Action::Quit),

        KeyCode::Enter | KeyCode::Char('e') => Some(Action::SelectWorklogForEdit),
        KeyCode::Char('d') => Some(Action::SelectWorklogForDelete),

        KeyCode::Down | KeyCode::Char('j') => Some(Action::SelectNext),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::SelectPrevious),

        _ => None,
    }
}
