use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type BoardId = u64;
pub type IssueId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: BoardId,
    pub name: String,
    pub project_key: String,
    pub board_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueStatus {
    Todo,
    InProgress,
    Done,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub key: IssueId,
    pub summary: String,
    pub description: Option<String>,
    pub status: IssueStatus,
    pub assignee: Option<String>,
    pub priority: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worklog {
    pub issue_key: IssueId,
    pub time_spent_seconds: u64,
    pub comment: Option<String>,
    pub started_at: DateTime<Utc>,
}
