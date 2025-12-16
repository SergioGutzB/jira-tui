use crate::domain::errors::{AppError, Result};
use std::env;

/// Holds the configuration required to authenticate with Jira.
#[derive(Clone)]
pub struct JiraConfig {
    pub base_url: String,
    pub email: String,
    pub api_token: String,
}

impl JiraConfig {
    /// Loads configuration from environment variables.
    ///
    /// # Variables required:
    /// - `JIRA_BASE_URL`
    /// - `JIRA_EMAIL`
    /// - `JIRA_API_TOKEN`
    pub fn from_env() -> Result<Self> {
        let base_url = env::var("JIRA_BASE_URL")
            .map_err(|_| AppError::ConfigError("JIRA_BASE_URL not set".to_string()))?;

        let email = env::var("JIRA_EMAIL")
            .map_err(|_| AppError::ConfigError("JIRA_EMAIL not set".to_string()))?;

        let api_token = env::var("JIRA_API_TOKEN")
            .map_err(|_| AppError::ConfigError("JIRA_API_TOKEN not set".to_string()))?;

        Ok(Self {
            base_url,
            email,
            api_token,
        })
    }
}
