mod application;
mod domain;
mod infrastructure;
mod ui;

use crossterm::event::KeyCode;
use dotenv::dotenv;
use log::error;
use std::sync::Arc;

use crate::application::use_cases::GetBoardsUseCase;
use crate::infrastructure::config::JiraConfig;
use crate::infrastructure::jira::client::JiraClient;
use crate::ui::app::{Action, App};
use crate::ui::events::{Event, EventHandler};
use crate::ui::tui;
use crate::ui::ui::render;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    // 1. Infrastructure Setup
    let config = JiraConfig::from_env().expect("Failed to load .env config");
    let jira_client = JiraClient::new(config)?;
    let repo = Arc::new(jira_client);

    // 2. Application Layer Setup (Use Cases)
    let get_boards_uc = Arc::new(GetBoardsUseCase::new(repo.clone()));

    // 3. UI Initialization
    let mut app = App::new();
    let mut terminal = tui::init()?;
    let mut events = EventHandler::new(250); // Tick every 250ms

    // Channel for Async Actions (Use Cases -> Main Loop)
    // The UI loop sends requests (sync), background tasks send responses (async) via this channel.
    let (action_tx, mut action_rx) = tokio::sync::mpsc::unbounded_channel();

    // --- MAIN LOOP ---
    loop {
        // A. Render
        terminal.draw(|frame| render(&app, frame))?;

        // B. Handle Inputs & Events
        // We select between user input (keyboard) and internal app messages (API responses)
        tokio::select! {
            // 1. User Inputs (Keyboard / Tick)
            Some(event) = events.next() => {
                match event {
                    Event::Key(key) => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                app.update(Action::Quit);
                            }
                            KeyCode::Char('b') => {
                                // Trigger: Load Boards
                                app.is_loading = true;
                                app.update(Action::LoadBoards); // Update UI state immediately

                                // Spawn async task
                                let uc = get_boards_uc.clone();
                                let tx = action_tx.clone();

                                tokio::spawn(async move {
                                    match uc.execute().await {
                                        Ok(boards) => {
                                            let _ = tx.send(Action::BoardsLoaded(boards));
                                        }
                                        Err(e) => {
                                            error!("Failed to fetch boards: {}", e);
                                            // Ideally send Action::Error(e) here
                                        }
                                    }
                                });
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                app.update(Action::SelectNext);
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                app.update(Action::SelectPrevious);
                            }
                            _ => {}
                        }
                    }
                    Event::Tick => {
                        app.update(Action::Tick);
                    }
                    _ => {}
                }
            }

            // 2. Application Actions (Responses from Async Tasks)
            Some(action) = action_rx.recv() => {
                app.update(action);
            }
        }

        // C. Check Quit Condition
        if app.should_quit {
            break;
        }
    }

    // 4. Cleanup
    tui::restore()?;
    Ok(())
}
