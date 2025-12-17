use crate::domain::errors::Result;
use crate::domain::models::{Board, BoardId, Issue, IssueFilter, Paginated, Worklog, WorklogEntry};
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

pub struct GetBacklogUseCase {
    repository: Arc<dyn JiraRepository>,
}

impl GetBacklogUseCase {
    pub fn new(repository: Arc<dyn JiraRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        board_id: BoardId,
        start_at: u64,
        max_results: u64,
        filter: IssueFilter,
    ) -> Result<Paginated<Issue>> {
        self.repository
            .get_issues_by_board(board_id, start_at, max_results, filter)
            .await
    }
}

pub struct AddWorklogUseCase {
    repository: Arc<dyn JiraRepository>,
}

impl AddWorklogUseCase {
    pub fn new(repository: Arc<dyn JiraRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, worklog: Worklog) -> Result<()> {
        self.repository.add_worklog(worklog).await
    }
}

pub struct GetWorklogsUseCase {
    repository: Arc<dyn JiraRepository>,
}

impl GetWorklogsUseCase {
    pub fn new(repository: Arc<dyn JiraRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, issue_key: &str, start_at: u64, max_results: u64) -> Result<Paginated<WorklogEntry>> {
        self.repository.get_worklogs(issue_key, start_at, max_results).await
    }
}

pub struct UpdateWorklogUseCase {
    repository: Arc<dyn JiraRepository>,
}

impl UpdateWorklogUseCase {
    pub fn new(repository: Arc<dyn JiraRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, issue_key: &str, worklog_id: &str, worklog: Worklog) -> Result<()> {
        self.repository.update_worklog(issue_key, worklog_id, worklog).await
    }
}

pub struct DeleteWorklogUseCase {
    repository: Arc<dyn JiraRepository>,
}

impl DeleteWorklogUseCase {
    pub fn new(repository: Arc<dyn JiraRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, issue_key: &str, worklog_id: &str) -> Result<()> {
        self.repository.delete_worklog(issue_key, worklog_id).await
    }
}
