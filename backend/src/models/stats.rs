use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult)]
pub struct FunctionRunCount {
    pub count: i64,
    pub name: String,
}

#[derive(Debug, FromQueryResult)]
pub struct UsageOverTime {
    pub timestamp: i64,
    pub total_invocations: i64,
}

#[derive(Debug, FromQueryResult)]
pub struct RawDashboardMetrics {
    pub current_invocations: i64,
    pub prev_invocations: i64,
    pub current_errors: i64,
    pub prev_errors: i64,
    pub current_latency: Option<f64>, // Option because AVG is null if 0 runs
    pub prev_latency: Option<f64>,
}