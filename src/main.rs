mod application;
mod domain;
mod infrastructure;
mod ui;

use crossterm::event::KeyCode;
use dotenv::dotenv;
use log::error;
use std::sync::Arc;

use crate::application::use_cases::{GetBacklogUseCase, GetBoardsUseCase}; // Imported GetBacklogUseCase
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
    let get_backlog_uc = Arc::new(GetBacklogUseCase::new(repo.clone())); // New instance

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
                            KeyCode::Char('q') | KeyCode::Esc => {
                                // Simple navigation logic: Esc goes back, or quits
                                match app.current_screen {
                                    CurrentScreen::Backlog => {
                                        app.update(Action::GoToBoards);
                                    },
                                    _ => app.update(Action::Quit),
                                }
                            }
                            KeyCode::Char('b') => {
                                // Reload boards shortcut
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
                            KeyCode::Enter => {
                                match app.current_screen {
                                    CurrentScreen::BoardsList => {
                                        if let Some(board) = app.get_selected_board() {
                                            let board_id = board.id;
                                            app.update(Action::LoadIssues(board_id));

                                            // Spawn fetch issues task
                                            let uc = get_backlog_uc.clone();
                                            let tx = action_tx.clone();
                                            tokio::spawn(async move {
                                                match uc.execute(board_id).await {
                                                    Ok(issues) => { let _ = tx.send(Action::IssuesLoaded(issues)); }
                                                    Err(e) => error!("Error loading issues: {}", e),
                                                }
                                            });
                                        }
                                    }
                                    // Future: Enter on Issue -> Show Details
                                    _ => {}
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => app.update(Action::SelectNext),
                            KeyCode::Up | KeyCode::Char('k') => app.update(Action::SelectPrevious),
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
