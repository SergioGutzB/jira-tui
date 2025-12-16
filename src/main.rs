mod application;
mod domain;
mod infrastructure;
mod ui;

use dotenv::dotenv;
use log::{error, info};
use std::sync::Arc;

use crate::application::use_cases::GetBoardsUseCase;
use crate::infrastructure::config::JiraConfig;
use crate::infrastructure::jira::client::JiraClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup logging and environment
    dotenv().ok();
    env_logger::init();

    // 2. Infrastructure Layer Setup
    let config = JiraConfig::from_env().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        e
    })?;

    info!("Initializing Jira Client with host: {}", config.base_url);

    let jira_client = JiraClient::new(config)?;

    // 3. Dependency Injection
    // Wrap the concrete client in an Arc<dyn Trait> for shared ownership.
    let repo = Arc::new(jira_client);

    // 4. Application Layer Setup
    // Inject the repository into the Use Case.
    let get_boards = GetBoardsUseCase::new(repo.clone());

    // --- TEMPORARY SANITY CHECK ---
    // This block verifies the architecture before we implement the TUI.
    println!("🚀 Fetching boards via Use Case...");
    match get_boards.execute().await {
        Ok(boards) => {
            println!("✅ Success! Boards found via UseCase:");
            for board in boards {
                println!("   * [ID: {}] {}", board.id, board.name);
            }
        }
        Err(e) => error!("❌ Error executing use case: {}", e),
    }

    Ok(())
}
