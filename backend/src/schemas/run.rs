use serde::Serialize;
use uuid::Uuid;

use crate::models::{function, run};

#[derive(Serialize)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

impl From<run::Status> for Status {
    fn from(value: run::Status) -> Self {
        match value {
            run::Status::Success => Self::Success,
            run::Status::Failure => Self::Failure,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResponse {
    pub function_name: String,
    pub run_id: Uuid,
    pub status: Status,
    pub triggered_at: i64,
    pub started_at: i64,
    pub finished_at: i64,
}

impl From<(run::Model, Option<function::Model>)> for RunResponse {
    fn from((run, function_opt): (run::Model, Option<function::Model>)) -> Self {
        Self {
            function_name: function_opt.map(|f| f.name).unwrap_or_else(|| "Unknown".to_string()),
            run_id: run.uuid,
            status: run.status.clone().into(),
            triggered_at: run.triggered_at.timestamp_millis(),
            started_at: run.started_at.timestamp_millis(),
            finished_at: run.finish_time.timestamp_millis(),
        }
    }
}
