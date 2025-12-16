use crate::domain::errors::Result;
use crate::domain::models::{Board, BoardId, Issue, IssueFilter, Worklog};
use async_trait::async_trait;

#[async_trait]
pub trait JiraRepository: Send + Sync {
    async fn get_boards(&self) -> Result<Vec<Board>>;

    /// Fetches a page of issues with filters.
    ///
    /// # Arguments
    /// * `board_id` - The ID of the board.
    /// * `start_at` - The pagination offset (e.g., 0, 20, 40).
    /// * `max_results` - How many items to fetch (e.g., 20).
    /// * `filter` - Criteria for JQL generation.
    async fn get_issues_by_board(
        &self,
        board_id: BoardId,
        start_at: u64,
        max_results: u64,
        filter: IssueFilter,
    ) -> Result<Vec<Issue>>;

    async fn add_worklog(&self, worklog: Worklog) -> Result<()>;
    async fn transition_issue(&self, issue_key: &str, transition_id: &str) -> Result<()>;
}
