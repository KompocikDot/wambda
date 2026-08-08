use serde::Serialize;

use crate::models;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Invocation {
    count: i64,
    name: String,
}

impl From<models::stats::FunctionRunCount> for Invocation {
    fn from(value: models::stats::FunctionRunCount) -> Self {
        Self {
            count: value.count,
            name: value.name,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsagePoint {
    pub timestamp: i64,
    pub total_invocations: i64,
}

impl From<models::stats::UsageOverTime> for UsagePoint {
    fn from(value: models::stats::UsageOverTime) -> Self {
        Self {
            timestamp: value.timestamp,
            total_invocations: value.total_invocations,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStats {
    pub total_invocations: i64,
    pub invocations_change_pct: f64,
    
    pub error_rate_pct: f64,
    pub error_rate_change_pct: f64,
    
    pub avg_latency_ms: f64,
    pub latency_change_ms: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsResponse {
    pub invocations: Vec<Invocation>,
    pub usage_over_time: Vec<UsagePoint>,
    pub metrics: DashboardStats
}