use crate::domain::errors::Result;
use crate::domain::models::{Board, BoardId, Issue, IssueFilter}; // Added IssueFilter
use crate::domain::repositories::JiraRepository;
use std::sync::Arc;

/// Use Case: Retrieve all visible boards for the authenticated user.
pub struct GetBoardsUseCase {
    repository: Arc<dyn JiraRepository>,
}

impl GetBoardsUseCase {
    pub fn new(repository: Arc<dyn JiraRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Board>> {
        self.repository.get_boards().await
    }
}

/// Use Case: Retrieve issues with pagination and filtering.
pub struct GetBacklogUseCase {
    repository: Arc<dyn JiraRepository>,
}

impl GetBacklogUseCase {
    pub fn new(repository: Arc<dyn JiraRepository>) -> Self {
        Self { repository }
    }

    /// Fetches a page of issues.
    ///
    /// # Arguments
    /// * `board_id` - The target board.
    /// * `start_at` - Pagination offset.
    /// * `max_results` - Number of items to retrieve.
    /// * `filter` - Search criteria (assignee, order, etc).
    pub async fn execute(
        &self,
        board_id: BoardId,
        start_at: u64,
        max_results: u64,
        filter: IssueFilter,
    ) -> Result<Vec<Issue>> {
        self.repository
            .get_issues_by_board(board_id, start_at, max_results, filter)
            .await
    }
}
