mod application;
mod domain;
mod infrastructure;
mod ui;

use dotenv::dotenv;
use std::sync::Arc;

use crate::application::use_cases::{GetBacklogUseCase, GetBoardsUseCase};
use crate::infrastructure::config::JiraConfig;
use crate::infrastructure::jira::client::JiraClient;
use crate::ui::app::{Action, App};
use crate::ui::events::{Event, EventHandler};
use crate::ui::handlers;
use crate::ui::keys;
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
            // A. USER INPUT
            Some(event) = events.next() => {
                match event {
                    Event::Key(key) => {
                        if let Some(action) = keys::from_event(key, &app) {
                            // Handle side effects (async network calls)
                            handlers::handle_side_effects(
                                &action,
                                get_boards_uc.clone(),
                                get_backlog_uc.clone(),
                                action_tx.clone(),
                            );

                            // Handle filter application
                            if matches!(action, Action::ApplyFilter) {
                                handlers::handle_filter_application(
                                    &app,
                                    get_backlog_uc.clone(),
                                    action_tx.clone(),
                                );
                            }

                            // Update UI state
                            app.update(action.clone());

                            // Check if infinite scroll should trigger
                            handlers::check_infinite_scroll(
                                &app,
                                get_backlog_uc.clone(),
                                action_tx.clone(),
                            );
                        }
                    }
                    Event::Tick => app.update(Action::Tick),
                    _ => {}
                }
            }

            // B. ASYNC BACKGROUND TASKS
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
