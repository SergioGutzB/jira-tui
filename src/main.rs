mod application;
mod domain;
mod infrastructure;
mod ui;

use dotenv::dotenv;
use log::error;
use std::sync::Arc;

use crate::application::use_cases::{GetBacklogUseCase, GetBoardsUseCase};
use crate::infrastructure::config::JiraConfig;
use crate::infrastructure::jira::client::JiraClient;
use crate::ui::app::{Action, App, CurrentScreen};
use crate::ui::events::{Event, EventHandler};
use crate::ui::keys;
use crate::ui::tui;
use crate::ui::ui::render; // Importamos el mapeador

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
                        // 1. Map Key -> Action (Pure Logic)
                        if let Some(action) = keys::from_event(key, &app) {

                            // 2. Handle Side Effects (Async Network Calls) based on the Action
                            match &action {
                                Action::LoadBoards => {
                                    let uc = get_boards_uc.clone();
                                    let tx = action_tx.clone();
                                    tokio::spawn(async move {
                                        match uc.execute().await {
                                            Ok(boards) => { let _ = tx.send(Action::BoardsLoaded(boards)); }
                                            Err(e) => error!("Error loading boards: {}", e),
                                        }
                                    });
                                }
                                Action::LoadIssues(board_id) => {
                                    let uc = get_backlog_uc.clone();
                                    let tx = action_tx.clone();
                                    let bid = *board_id;

                                    tokio::spawn(async move {
                                        let filter = crate::domain::models::IssueFilter::default_active_user();
                                        // Load Page 1 (0..20)
                                        match uc.execute(bid, 0, 20, filter).await {
                                            Ok(p) => { let _ = tx.send(Action::IssuesLoaded(p)); }
                                            Err(e) => error!("Error loading issues: {}", e),
                                        }
                                    });
                                }
                                _ => {}
                            }

                            // 3. Update UI State (Synchronous)
                            app.update(action.clone());

                            // 4. Post-Update Logic (e.g., Infinite Scroll Trigger)
                            // This checks if the state change requires fetching more data
                            check_infinite_scroll(&mut app, get_backlog_uc.clone(), action_tx.clone());
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

/// Helper to handle infinite scroll logic separately from key bindings
fn check_infinite_scroll(
    app: &mut App,
    uc: Arc<GetBacklogUseCase>,
    tx: tokio::sync::mpsc::UnboundedSender<Action>,
) {
    if app.current_screen == CurrentScreen::Backlog
        && !app.is_loading
        && app.issues.len() < app.total_issues as usize
        && app.selected_issue_index >= app.issues.len().saturating_sub(2)
    {
        if let Some(board_id) = app.current_board_id {
            let start_at = app.issues.len() as u64;
            app.is_loading = true;

            tokio::spawn(async move {
                let filter = crate::domain::models::IssueFilter::default_active_user();
                match uc.execute(board_id, start_at, 20, filter).await {
                    Ok(p) => {
                        let _ = tx.send(Action::IssuesLoaded(p));
                    }
                    Err(e) => error!("Pagination err: {}", e),
                }
            });
        }
    }
}
