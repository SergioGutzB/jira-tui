use crate::domain::errors::{AppError, Result};
use crate::domain::models::{Board, BoardId, Issue, IssueFilter, Worklog};
use crate::domain::repositories::JiraRepository;
use crate::infrastructure::config::JiraConfig;
use crate::infrastructure::jira::dtos::{BoardResponseDto, IssueSearchResponseDto};
use async_trait::async_trait;
use reqwest::{Client, StatusCode};

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
        // ... (Tu implementación de paginación de boards 'Fetch All' está bien aquí
        // porque los boards suelen ser pocos, o podemos aplicar la misma lógica si tienes miles).
        // Por simplicidad, mantengamos el loop anterior de boards o limítalo a 50.
        // Aquí dejo la versión simple (paginada internamente o limitada) para no alargar:

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
    ) -> Result<Vec<Issue>> {
        // 1. Construct JQL
        let mut jql_parts = Vec::new();

        // By default, the board endpoint filters by the board context.
        // We add extra filters.

        if let Some(assignee) = &filter.assignee {
            // "currentUser()" is a Jira function, handled directly.
            // Other names might need quoting depending on spaces, but simple names work.
            jql_parts.push(format!("assignee = {}", assignee));
        }

        if let Some(status) = &filter.status {
            jql_parts.push(format!("status = \"{}\"", status));
        }

        // Combine filters with AND
        let jql_query = if jql_parts.is_empty() {
            String::new()
        } else {
            jql_parts.join(" AND ")
        };

        // Add Order By
        let final_jql = if let Some(order) = &filter.order_by {
            if jql_query.is_empty() {
                format!("ORDER BY {}", order)
            } else {
                format!("{} ORDER BY {}", jql_query, order)
            }
        } else {
            jql_query
        };

        // 2. Build URL
        // Endpoint: /rest/agile/1.0/board/{boardId}/issue
        // Params: startAt, maxResults, jql
        let url = format!("{}/rest/agile/1.0/board/{}/issue", self.base_url, board_id);

        // 3. Request
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
        Err(AppError::Unknown("Not implemented yet".to_string()))
    }

    async fn transition_issue(&self, _issue_key: &str, _transition_id: &str) -> Result<()> {
        Err(AppError::Unknown("Not implemented yet".to_string()))
    }
}
