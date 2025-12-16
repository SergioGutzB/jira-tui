use crate::domain::errors::{AppError, Result};
use crate::domain::models::{Board, BoardId, Issue, Worklog};
use crate::domain::repositories::JiraRepository;
use crate::infrastructure::config::JiraConfig;
use crate::infrastructure::jira::dtos::{BoardResponseDto, IssueSearchResponseDto};
use async_trait::async_trait;
use reqwest::{Client, StatusCode};

/// Concrete implementation of the JiraRepository using reqwest.
pub struct JiraClient {
    client: Client,
    base_url: String,
}

impl JiraClient {
    /// Creates a new instance of JiraClient with authenticated headers.
    pub fn new(config: JiraConfig) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .basic_auth(config.email, Some(config.api_token))
            .build()
            .map_err(|e| AppError::ConfigError(e.to_string()))?;

        Ok(Self {
            client,
            base_url: config.base_url.trim_end_matches('/').to_string(),
        })
    }
}

#[async_trait]
impl JiraRepository for JiraClient {
    async fn get_boards(&self) -> Result<Vec<Board>> {
        let url = format!("{}/rest/agile/1.0/board", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::ApiError(e.to_string()))?;

        match response.status() {
            StatusCode::OK => {
                let dto: BoardResponseDto = response
                    .json()
                    .await
                    .map_err(|e| AppError::ApiError(format!("Failed to parse boards: {}", e)))?;

                Ok(dto.values.into_iter().map(Into::into).collect())
            }
            StatusCode::UNAUTHORIZED => Err(AppError::Unauthorized),
            _ => Err(AppError::ApiError(format!(
                "Jira API Error: {}",
                response.status()
            ))),
        }
    }

    async fn get_issues_by_board(&self, board_id: BoardId) -> Result<Vec<Issue>> {
        // Fetching backlog issues for the board
        let url = format!("{}/rest/agile/1.0/board/{}/issue", self.base_url, board_id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::ApiError(e.to_string()))?;

        match response.status() {
            StatusCode::OK => {
                let dto: IssueSearchResponseDto = response
                    .json()
                    .await
                    .map_err(|e| AppError::ApiError(format!("Failed to parse issues: {}", e)))?;

                Ok(dto.issues.into_iter().map(Into::into).collect())
            }
            StatusCode::UNAUTHORIZED => Err(AppError::Unauthorized),
            StatusCode::NOT_FOUND => {
                Err(AppError::NotFound(format!("Board {} not found", board_id)))
            }
            _ => Err(AppError::ApiError(format!(
                "Jira API Error: {}",
                response.status()
            ))),
        }
    }

    async fn add_worklog(&self, _worklog: Worklog) -> Result<()> {
        // Implementation postponed for later step
        Err(AppError::Unknown("Not implemented yet".to_string()))
    }

    async fn transition_issue(&self, _issue_key: &str, _transition_id: &str) -> Result<()> {
        // Implementation postponed for later step
        Err(AppError::Unknown("Not implemented yet".to_string()))
    }
}
