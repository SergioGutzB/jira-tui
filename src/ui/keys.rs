use crate::ui::app::{Action, App, CurrentScreen};
use crossterm::event::{KeyCode, KeyEvent};

/// Maps a physical key event to an application action based on context.
pub fn from_event(key: KeyEvent, app: &App) -> Option<Action> {
    match app.current_screen {
        CurrentScreen::Dashboard | CurrentScreen::BoardsList => match_boards_keys(key, app),
        CurrentScreen::Backlog => match_backlog_keys(key, app),
        CurrentScreen::IssueDetail => match_detail_keys(key),
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
        // Navigation Back
        KeyCode::Esc => Some(Action::GoToBoards),
        KeyCode::Char('b') => Some(Action::GoToBoards), // Alias

        // Context Specific
        KeyCode::Enter => Some(Action::ViewIssueDetail),
        KeyCode::Char('q') => Some(Action::Quit), // Or Action::GoToBoards if preferred

        // List Navigation
        KeyCode::Down | KeyCode::Char('j') => Some(Action::SelectNext),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::SelectPrevious),

        _ => None,
    }
}

fn match_detail_keys(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Esc => Some(Action::GoToBacklog),
        KeyCode::Char('q') => Some(Action::Quit),

        // Scroll
        KeyCode::Down | KeyCode::Char('j') => Some(Action::SelectNext),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::SelectPrevious),

        _ => None,
    }
}
