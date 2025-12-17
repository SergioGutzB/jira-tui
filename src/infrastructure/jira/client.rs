use crate::domain::errors::{AppError, Result};
use crate::domain::models::{Board, BoardId, Issue, IssueFilter, Paginated, Worklog, WorklogEntry};
use crate::domain::repositories::JiraRepository;
use crate::infrastructure::config::JiraConfig;
use crate::infrastructure::jira::dtos::{BoardResponseDto, IssueSearchResponseDto, WorklogResponseDto};
use async_trait::async_trait;
use reqwest::{Client, StatusCode};

/// Converts seconds to Jira time format (e.g., "1h 30m", "2h", "45m")
fn format_time_spent(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;

    match (hours, minutes) {
        (0, 0) => "1m".to_string(),
        (0, m) => format!("{}m", m),
        (h, 0) => format!("{}h", h),
        (h, m) => format!("{}h {}m", h, m),
    }
}

pub struct JiraClient {
    client: Client,
    base_url: String,
    email: String,
    api_token: String,
}

impl JiraClient {
    pub fn new(config: JiraConfig) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| AppError::ConfigError(e.to_string()))?;

        Ok(Self {
            client,
            base_url: config.base_url.trim_end_matches('/').to_string(),
            email: config.email,
            api_token: config.api_token,
        })
    }
}

#[async_trait]
impl JiraRepository for JiraClient {
    async fn get_boards(&self) -> Result<Vec<Board>> {
        let url = format!("{}/rest/agile/1.0/board?maxResults=100", self.base_url);
        let response = self
            .client
            .get(&url)
            .basic_auth(&self.email, Some(&self.api_token))
            .send()
            .await
            .map_err(|e| AppError::ApiError(e.to_string()))?;

        if response.status() == StatusCode::OK {
            let dto: BoardResponseDto = response
                .json()
                .await
                .map_err(|e| AppError::ApiError(format!("Parse error: {}", e)))?;
            Ok(dto.values.into_iter().map(Into::into).collect())
        } else {
            Err(AppError::ApiError(format!("Error: {}", response.status())))
        }
    }

    async fn get_issues_by_board(
        &self,
        board_id: BoardId,
        start_at: u64,
        max_results: u64,
        filter: IssueFilter,
    ) -> Result<Paginated<Issue>> {
        let mut jql_parts = Vec::new();
        if let Some(assignee) = &filter.assignee {
            jql_parts.push(format!("assignee = {}", assignee));
        }
        if let Some(status) = &filter.status {
            jql_parts.push(format!("status = \"{}\"", status));
        }
        let jql_query = if jql_parts.is_empty() {
            String::new()
        } else {
            jql_parts.join(" AND ")
        };

        let final_jql = if let Some(order) = &filter.order_by {
            if jql_query.is_empty() {
                format!("ORDER BY {}", order)
            } else {
                format!("{} ORDER BY {}", jql_query, order)
            }
        } else {
            jql_query
        };

        let url = format!("{}/rest/agile/1.0/board/{}/issue", self.base_url, board_id);

        let response = self
            .client
            .get(&url)
            .basic_auth(&self.email, Some(&self.api_token))
            .query(&[
                ("startAt", start_at.to_string()),
                ("maxResults", max_results.to_string()),
                ("jql", final_jql),
            ])
            .send()
            .await
            .map_err(|e| AppError::ApiError(e.to_string()))?;

        match response.status() {
            StatusCode::OK => {
                let dto: IssueSearchResponseDto = response
                    .json()
                    .await
                    .map_err(|e| AppError::ApiError(format!("Failed to parse issues: {}", e)))?;

                let issues: Vec<Issue> = dto.issues.into_iter().map(Into::into).collect();

                Ok(Paginated::new(
                    issues,
                    dto.total,
                    dto.start_at,
                    dto.max_results,
                ))
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

    async fn add_worklog(&self, worklog: Worklog) -> Result<()> {
        let url = format!(
            "{}/rest/api/3/issue/{}/worklog",
            self.base_url, worklog.issue_key
        );

        let started = worklog.started_at.format("%Y-%m-%dT%H:%M:%S%.3f%z").to_string();

        let mut payload = serde_json::json!({
            "timeSpentSeconds": worklog.time_spent_seconds,
            "started": started,
        });

        if let Some(comment_text) = worklog.comment {
            if !comment_text.is_empty() {
                payload["comment"] = serde_json::json!({
                    "type": "doc",
                    "version": 1,
                    "content": [{
                        "type": "paragraph",
                        "content": [{
                            "type": "text",
                            "text": comment_text
                        }]
                    }]
                });
            }
        }

        log::debug!("Worklog URL: {}", url);
        log::debug!("Worklog payload: {}", serde_json::to_string_pretty(&payload).unwrap());

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.email, Some(&self.api_token))
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::ApiError(format!("Failed to add worklog: {}", e)))?;

        match response.status() {
            StatusCode::CREATED | StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => Err(AppError::Unauthorized),
            StatusCode::NOT_FOUND => Err(AppError::NotFound(format!(
                "Issue {} not found",
                worklog.issue_key
            ))),
            _ => Err(AppError::ApiError(format!(
                "Failed to add worklog: {}",
                response.status()
            ))),
        }
    }

    async fn get_worklogs(&self, issue_key: &str, start_at: u64, max_results: u64) -> Result<Paginated<WorklogEntry>> {
        let url = format!(
            "{}/rest/api/3/issue/{}/worklog",
            self.base_url, issue_key
        );

        let response = self
            .client
            .get(&url)
            .basic_auth(&self.email, Some(&self.api_token))
            .query(&[
                ("startAt", start_at.to_string()),
                ("maxResults", max_results.to_string()),
            ])
            .send()
            .await
            .map_err(|e| AppError::ApiError(format!("Failed to get worklogs: {}", e)))?;

        match response.status() {
            StatusCode::OK => {
                let dto: WorklogResponseDto = response
                    .json()
                    .await
                    .map_err(|e| AppError::ApiError(format!("Failed to parse worklogs: {}", e)))?;

                let worklogs: Vec<WorklogEntry> = dto
                    .worklogs
                    .into_iter()
                    .map(|w| w.to_worklog_entry(issue_key.to_string()))
                    .collect();

                Ok(Paginated::new(
                    worklogs,
                    dto.total,
                    dto.start_at,
                    dto.max_results,
                ))
            }
            StatusCode::UNAUTHORIZED => Err(AppError::Unauthorized),
            StatusCode::NOT_FOUND => Err(AppError::NotFound(format!(
                "Issue {} not found",
                issue_key
            ))),
            _ => Err(AppError::ApiError(format!(
                "Failed to get worklogs: {}",
                response.status()
            ))),
        }
    }

    async fn update_worklog(&self, issue_key: &str, worklog_id: &str, worklog: Worklog) -> Result<()> {
        let url = format!(
            "{}/rest/api/3/issue/{}/worklog/{}",
            self.base_url, issue_key, worklog_id
        );

        let started = worklog.started_at.format("%Y-%m-%dT%H:%M:%S%.3f%z").to_string();

        let mut payload = serde_json::json!({
            "timeSpentSeconds": worklog.time_spent_seconds,
            "started": started,
        });

        if let Some(comment_text) = worklog.comment {
            if !comment_text.is_empty() {
                payload["comment"] = serde_json::json!({
                    "type": "doc",
                    "version": 1,
                    "content": [{
                        "type": "paragraph",
                        "content": [{
                            "type": "text",
                            "text": comment_text
                        }]
                    }]
                });
            }
        }

        let response = self
            .client
            .put(&url)
            .basic_auth(&self.email, Some(&self.api_token))
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::ApiError(format!("Failed to update worklog: {}", e)))?;

        match response.status() {
            StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => Err(AppError::Unauthorized),
            StatusCode::NOT_FOUND => Err(AppError::NotFound(format!(
                "Worklog {} not found",
                worklog_id
            ))),
            _ => Err(AppError::ApiError(format!(
                "Failed to update worklog: {}",
                response.status()
            ))),
        }
    }

    async fn delete_worklog(&self, issue_key: &str, worklog_id: &str) -> Result<()> {
        let url = format!(
            "{}/rest/api/3/issue/{}/worklog/{}",
            self.base_url, issue_key, worklog_id
        );

        let response = self
            .client
            .delete(&url)
            .basic_auth(&self.email, Some(&self.api_token))
            .send()
            .await
            .map_err(|e| AppError::ApiError(format!("Failed to delete worklog: {}", e)))?;

        match response.status() {
            StatusCode::NO_CONTENT | StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => Err(AppError::Unauthorized),
            StatusCode::NOT_FOUND => Err(AppError::NotFound(format!(
                "Worklog {} not found",
                worklog_id
            ))),
            _ => Err(AppError::ApiError(format!(
                "Failed to delete worklog: {}",
                response.status()
            ))),
        }
    }

    async fn transition_issue(&self, _issue_key: &str, _transition_id: &str) -> Result<()> {
        Err(AppError::Unknown("Not implemented yet".to_string()))
    }
}
