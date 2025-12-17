mod application;
mod domain;
mod infrastructure;
mod ui;

use crossterm::event::KeyCode;
use dotenv::dotenv;
use log::error;
use std::sync::Arc;

use crate::application::use_cases::{GetBacklogUseCase, GetBoardsUseCase};
use crate::domain::models::IssueFilter;
use crate::infrastructure::config::JiraConfig;
use crate::infrastructure::jira::client::JiraClient;
use crate::ui::app::{Action, App, CurrentScreen};
use crate::ui::events::{Event, EventHandler};
use crate::ui::tui;
use crate::ui::ui::render;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    // 1. Infrastructure
    let config = JiraConfig::from_env().expect("Failed to load .env config");
    let jira_client = JiraClient::new(config)?;
    let repo = Arc::new(jira_client);

    // 2. Use Cases
    let get_boards_uc = Arc::new(GetBoardsUseCase::new(repo.clone()));
    let get_backlog_uc = Arc::new(GetBacklogUseCase::new(repo.clone()));

    // 3. UI Init
    let mut app = App::new();
    let mut terminal = tui::init()?;
    let mut events = EventHandler::new(250);

    let (action_tx, mut action_rx) = tokio::sync::mpsc::unbounded_channel();

    loop {
        terminal.draw(|frame| render(&app, frame))?;

        tokio::select! {
            Some(event) = events.next() => {
                match event {
                    Event::Key(key) => {
                        match key.code {
                            // GLOBAL QUIT
                            KeyCode::Char('q') => {
                                // If in detail or backlog, 'q' might mean quit app
                                // or we can enforce 'Esc' to go back first.
                                // For now, let's make 'q' always quit for speed.
                                app.update(Action::Quit);
                            }

                            // NAVIGATION BACK
                            KeyCode::Esc => {
                                match app.current_screen {
                                    CurrentScreen::IssueDetail => {
                                        // Go back to list
                                        app.current_screen = CurrentScreen::Backlog;
                                        app.vertical_scroll = 0;
                                    }
                                    CurrentScreen::Backlog => {
                                        app.update(Action::GoToBoards);
                                    }
                                    _ => app.update(Action::Quit),
                                }
                            }

                            // LOAD BOARDS
                            KeyCode::Char('b') => {
                                app.update(Action::LoadBoards);
                                let uc = get_boards_uc.clone();
                                let tx = action_tx.clone();
                                tokio::spawn(async move {
                                    match uc.execute().await {
                                        Ok(boards) => { let _ = tx.send(Action::BoardsLoaded(boards)); }
                                        Err(e) => error!("Error loading boards: {}", e),
                                    }
                                });
                            }

                            // SELECT / ENTER
                            KeyCode::Enter => {
                                match app.current_screen {
                                    CurrentScreen::BoardsList => {
                                        if let Some(board) = app.get_selected_board() {
                                            let board_id = board.id;
                                            app.update(Action::LoadIssues(board_id));

                                            let uc = get_backlog_uc.clone();
                                            let tx = action_tx.clone();

                                            // Primera página
                                            tokio::spawn(async move {
                                                let filter = IssueFilter::default_active_user();
                                                match uc.execute(board_id, 0, 20, filter).await {
                                                    Ok(p) => { let _ = tx.send(Action::IssuesLoaded(p)); }
                                                    Err(e) => error!("Error: {}", e),
                                                }
                                            });
                                        }
                                    }
                                    // ...
                                    _ => {}
                                }
                            }

                            // SCROLL / MOVE
                            KeyCode::Up | KeyCode::Char('k') => app.update(Action::SelectPrevious),

                            KeyCode::Down | KeyCode::Char('j') => {
                                app.update(Action::SelectNext);

                                // Detectar si necesitamos cargar más
                                if app.current_screen == CurrentScreen::Backlog
                                   && !app.is_loading
                                   && app.issues.len() < app.total_issues as usize
                                   && app.selected_issue_index >= app.issues.len().saturating_sub(2)
                                {
                                    if let Some(board_id) = app.current_board_id {
                                        let start_at = app.issues.len() as u64;
                                        app.is_loading = true; // Feedback visual inmediato

                                        let uc = get_backlog_uc.clone();
                                        let tx = action_tx.clone();

                                        tokio::spawn(async move {
                                            let filter = IssueFilter::default_active_user();
                                            // Pide los siguientes 20
                                            match uc.execute(board_id, start_at, 20, filter).await {
                                                Ok(p) => { let _ = tx.send(Action::IssuesLoaded(p)); }
                                                Err(e) => error!("Pagination Error: {}", e),
                                            }
                                        });
                                    }
                                }
                            }

                            _ => {}
                        }
                    }
                    Event::Tick => app.update(Action::Tick),
                    _ => {}
                }
            }
            Some(action) = action_rx.recv() => {
                app.update(action);
            }
        }

        if app.should_quit {
            break;
        }
    }

    tui::restore()?;
    Ok(())
}
