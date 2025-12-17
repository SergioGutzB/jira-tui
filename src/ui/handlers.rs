use chrono::{Utc, TimeZone, Local};
use log::error;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

use crate::application::use_cases::{AddWorklogUseCase, GetBacklogUseCase, GetBoardsUseCase};
use crate::domain::models::{IssueFilter, Worklog};
use crate::ui::app::{Action, App, CurrentScreen};

/// Handles side effects for actions that require async network calls.
/// This function spawns tokio tasks to avoid blocking the UI render loop.
pub fn handle_side_effects(
    action: &Action,
    get_boards_uc: Arc<GetBoardsUseCase>,
    get_backlog_uc: Arc<GetBacklogUseCase>,
    tx: UnboundedSender<Action>,
) {
    match action {
        Action::LoadBoards => {
            tokio::spawn(async move {
                match get_boards_uc.execute().await {
                    Ok(boards) => {
                        let _ = tx.send(Action::BoardsLoaded(boards));
                    }
                    Err(e) => error!("Error loading boards: {}", e),
                }
            });
        }

        Action::LoadIssues(board_id) => {
            let bid = *board_id;
            tokio::spawn(async move {
                let filter = IssueFilter::default_active_user();
                match get_backlog_uc.execute(bid, 0, 20, filter).await {
                    Ok(p) => {
                        let _ = tx.send(Action::IssuesLoaded(p));
                    }
                    Err(e) => error!("Error loading issues: {}", e),
                }
            });
        }

        Action::ApplyFilter => {
            // This will be handled by triggering a LoadIssues with new filter
            // The logic is in handle_filter_application
        }

        _ => {}
    }
}

/// Handles the filter application by reloading issues with new filter criteria.
pub fn handle_filter_application(
    app: &App,
    get_backlog_uc: Arc<GetBacklogUseCase>,
    tx: UnboundedSender<Action>,
) {
    if let Some(board_id) = app.current_board_id {
        let filter = IssueFilter::from_options(
            app.filter_assignee.clone(),
            app.filter_status.clone(),
            app.filter_order_by.clone(),
        );

        tokio::spawn(async move {
            match get_backlog_uc.execute(board_id, 0, 20, filter).await {
                Ok(p) => {
                    let _ = tx.send(Action::IssuesLoaded(p));
                }
                Err(e) => error!("Error applying filter: {}", e),
            }
        });
    }
}

/// Checks if infinite scroll should be triggered and loads more issues if needed.
///
/// This is called after state updates to check if the user has scrolled near
/// the bottom of the list and there are more items to load.
pub fn check_infinite_scroll(
    app: &App,
    get_backlog_uc: Arc<GetBacklogUseCase>,
    tx: UnboundedSender<Action>,
) {
    if app.current_screen == CurrentScreen::Backlog
        && !app.is_loading
        && app.issues.len() < app.total_issues as usize
        && app.selected_issue_index >= app.issues.len().saturating_sub(2)
    {
        if let Some(board_id) = app.current_board_id {
            let start_at = app.issues.len() as u64;

            let filter = IssueFilter::from_options(
                app.filter_assignee.clone(),
                app.filter_status.clone(),
                app.filter_order_by.clone(),
            );

            tokio::spawn(async move {
                match get_backlog_uc.execute(board_id, start_at, 20, filter).await {
                    Ok(p) => {
                        let _ = tx.send(Action::IssuesLoaded(p));
                    }
                    Err(e) => error!("Pagination error: {}", e),
                }
            });
        }
    }
}

/// Handles worklog submission by creating a Worklog and sending it to Jira.
pub fn handle_worklog_submission(
    app: &App,
    add_worklog_uc: Arc<AddWorklogUseCase>,
    tx: UnboundedSender<Action>,
) {
    if let Some(issue) = app.get_selected_issue() {
        let total_seconds = (app.worklog_time_hours as u64 * 3600) + (app.worklog_time_minutes as u64 * 60);

        if total_seconds == 0 {
            error!("Cannot log 0 time");
            return;
        }

        let started_at = match Local.with_ymd_and_hms(
            app.worklog_year as i32,
            app.worklog_month as u32,
            app.worklog_day as u32,
            app.worklog_hour as u32,
            app.worklog_minute as u32,
            0,
        ) {
            chrono::LocalResult::Single(dt) => dt.with_timezone(&Utc),
            _ => {
                error!("Invalid date/time");
                return;
            }
        };

        let worklog = Worklog {
            issue_key: issue.key.clone(),
            time_spent_seconds: total_seconds,
            comment: if app.worklog_comment.is_empty() {
                None
            } else {
                Some(app.worklog_comment.clone())
            },
            started_at,
        };

        let tx_clone = tx.clone();
        tokio::spawn(async move {
            match add_worklog_uc.execute(worklog).await {
                Ok(_) => {
                    let _ = tx.send(Action::ShowNotification(
                        "✅ Éxito".to_string(),
                        "Tiempo registrado correctamente".to_string(),
                        true,
                    ));
                    let _ = tx.send(Action::WorklogSubmitted);

                    // Auto-dismiss notification after 3 seconds
                    let tx_dismiss = tx_clone.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                        let _ = tx_dismiss.send(Action::HideNotification);
                    });
                }
                Err(e) => {
                    let _ = tx.send(Action::ShowNotification(
                        "❌ Error".to_string(),
                        format!("Error al registrar tiempo: {}", e),
                        false,
                    ));
                    error!("Error adding worklog: {}", e);

                    // Auto-dismiss error notification after 5 seconds
                    let tx_dismiss = tx_clone.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        let _ = tx_dismiss.send(Action::HideNotification);
                    });
                }
            }
        });
    }
}
