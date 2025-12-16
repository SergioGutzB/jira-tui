use crate::domain::models::{Board, Issue, IssueStatus};
use serde::Deserialize;

/// Represents the raw response from Jira when querying for boards.
#[derive(Deserialize)]
pub struct BoardResponseDto {
    /// List of values returned by the API.
    pub values: Vec<BoardDto>,
}

/// Represents a single Board object from the Jira API.
#[derive(Deserialize)]
pub struct BoardDto {
    pub id: u64,
    pub name: String,
    pub location: Option<ProjectLocationDto>,
    #[serde(rename = "type")]
    pub board_type: String,
}

#[derive(Deserialize)]
pub struct ProjectLocationDto {
    #[serde(rename = "projectKey")]
    pub project_key: Option<String>,
}

impl From<BoardDto> for Board {
    fn from(dto: BoardDto) -> Self {
        let project_key = dto
            .location
            .and_then(|l| l.project_key)
            .unwrap_or_else(|| "UNKNOWN".to_string());

        Board {
            id: dto.id,
            name: dto.name,
            project_key,
            board_type: dto.board_type,
        }
    }
}

/// Represents the raw response for a JQL search or backlog.
#[derive(Deserialize)]
pub struct IssueSearchResponseDto {
    pub issues: Vec<IssueDto>,
}

/// Represents the nested structure of a Jira Issue.
#[derive(Deserialize)]
pub struct IssueDto {
    pub key: String,
    pub fields: IssueFieldsDto,
}

#[derive(Deserialize)]
pub struct IssueFieldsDto {
    pub summary: String,
    pub description: Option<String>,
    pub status: StatusDto,
    pub priority: Option<PriorityDto>,
    pub assignee: Option<UserDto>,
    pub created: String,
    pub updated: String,
}

#[derive(Deserialize)]
pub struct StatusDto {
    pub name: String,
}

#[derive(Deserialize)]
pub struct PriorityDto {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UserDto {
    #[serde(rename = "displayName")]
    pub display_name: String,
}

impl From<IssueDto> for Issue {
    fn from(dto: IssueDto) -> Self {
        let status = match dto.fields.status.name.to_lowercase().as_str() {
            "to do" | "new" => IssueStatus::Todo,
            "in progress" => IssueStatus::InProgress,
            "done" | "closed" => IssueStatus::Done,
            _ => IssueStatus::Other(dto.fields.status.name),
        };

        let created_at =
            chrono::DateTime::parse_from_str(&dto.fields.created, "%Y-%m-%dT%H:%M:%S.%f%z")
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

        let updated_at =
            chrono::DateTime::parse_from_str(&dto.fields.updated, "%Y-%m-%dT%H:%M:%S.%f%z")
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

        Issue {
            key: dto.key,
            summary: dto.fields.summary,
            description: dto.fields.description,
            status,
            assignee: dto.fields.assignee.map(|u| u.display_name),
            priority: dto.fields.priority.map(|p| p.name),
            created_at,
            updated_at,
        }
    }
}
