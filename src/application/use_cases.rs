use crate::domain::errors::Result;
use crate::domain::models::{Board, BoardId, Issue};
use crate::domain::repositories::JiraRepository;
use std::sync::Arc;

/// Use Case: Retrieve all visible boards for the authenticated user.
///
/// # Performance Design
/// - Uses `Arc<dyn JiraRepository>` to share the repository instance cheaply across use cases.
/// - Returns full ownership of the `Vec<Board>` to the caller to avoid lifetime complications in the UI.
pub struct GetBoardsUseCase {
    repository: Arc<dyn JiraRepository>,
}

impl GetBoardsUseCase {
    pub fn new(repository: Arc<dyn JiraRepository>) -> Self {
        Self { repository }
    }

    /// Executes the use case.
    ///
    /// This function is async and non-blocking. It delegates the fetching
    /// to the infrastructure layer.
    pub async fn execute(&self) -> Result<Vec<Board>> {
        self.repository.get_boards().await
    }
}

/// Use Case: Retrieve issues from a specific board or backlog.
pub struct GetBacklogUseCase {
    repository: Arc<dyn JiraRepository>,
}

impl GetBacklogUseCase {
    pub fn new(repository: Arc<dyn JiraRepository>) -> Self {
        Self { repository }
    }

    /// Fetches issues associated with the given `board_id`.
    pub async fn execute(&self, board_id: BoardId) -> Result<Vec<Issue>> {
        self.repository.get_issues_by_board(board_id).await
    }
}
