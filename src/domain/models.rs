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

/// Represents the search criteria for issues.
#[derive(Debug, Clone, Default)]
pub struct IssueFilter {
    /// If Some, filters by this username. If "currentUser()", uses Jira's dynamic value.
    pub assignee: Option<String>,
    /// Filter by specific status name (e.g., "In Progress").
    pub status: Option<String>,
    /// JQL Order By clause. Default should be "updated DESC".
    pub order_by: Option<String>,
}

impl IssueFilter {
    /// Creates a default filter for the current user sorted by update time.
    pub fn default_active_user() -> Self {
        Self {
            assignee: Some("currentUser()".to_string()),
            status: None, // None means "All statuses"
            order_by: Some("updated DESC".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub start_at: u64,
    pub max_results: u64,
}

impl<T> Paginated<T> {
    pub fn new(items: Vec<T>, total: u64, start_at: u64, max_results: u64) -> Self {
        Self {
            items,
            total,
            start_at,
            max_results,
        }
    }
}
