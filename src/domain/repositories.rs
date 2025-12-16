use crate::domain::errors::Result;
use crate::domain::models::{Board, BoardId, Issue, Worklog};
use async_trait::async_trait;

#[async_trait]
pub trait JiraRepository: Send + Sync {
    async fn get_boards(&self) -> Result<Vec<Board>>;

    async fn get_issues_by_board(&self, board_id: BoardId) -> Result<Vec<Issue>>;

    async fn add_worklog(&self, worklog: Worklog) -> Result<()>;

    async fn transition_issue(&self, issue_key: &str, transition_id: &str) -> Result<()>;
}
