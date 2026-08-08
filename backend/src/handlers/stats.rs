use std::sync::Arc;

use anyhow::Context;
use axum::{Json, Router, extract::State, routing::get};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect,
    RelationTrait, sea_query::Expr,
};
use tokio::try_join;

use crate::{
    errors::AppError,
    exctractors::AuthUser,
    models::{
        function, run,
        stats::{FunctionRunCount, RawDashboardMetrics, UsageOverTime},
    },
    schemas::stats::{DashboardStats, StatsResponse},
    state::AppState,
};

pub fn stats_router() -> Router<Arc<AppState>> {
    return Router::new().route("/", get(main_page_stats));
}

async fn fetch_invocations_list(user_id: i32, db: &DatabaseConnection) -> Result<Vec<FunctionRunCount>, AppError> {
    let top_invocations: Vec<FunctionRunCount> = run::Entity::find()
        .select_only()
        .column(function::Column::Name)
        .column_as(run::Column::Id.count(), "count")
        .join(JoinType::InnerJoin, run::Relation::Function.def())
        .filter(function::Column::OwnerId.eq(user_id))
        .group_by(function::Column::Name)
        .order_by_desc(run::Column::Id.count())
        .limit(10)
        .into_model::<FunctionRunCount>()
        .all(db)
        .await
        .context("could not get top invocations from db")?;

    Ok(top_invocations)
}

async fn fetch_metrics(user_id: i32, db: &DatabaseConnection) -> Result<DashboardStats, AppError> {
    let now_ms = chrono::Utc::now().timestamp_millis();
    let last_24h_ms = now_ms - 86_400_000;
    let prev_24h_ms = last_24h_ms - 86_400_000;

    let raw_metrics: Option<RawDashboardMetrics> = run::Entity::find()
        .select_only()
        .column_as(Expr::cust(format!("CAST(COALESCE(SUM(CASE WHEN runs.triggered_at >= {} THEN 1 ELSE 0 END), 0) AS BIGINT)", last_24h_ms)), "current_invocations")
        .column_as(Expr::cust(format!("CAST(COALESCE(SUM(CASE WHEN runs.triggered_at < {} THEN 1 ELSE 0 END), 0) AS BIGINT)", last_24h_ms)), "prev_invocations")

        .column_as(Expr::cust(format!("CAST(COALESCE(SUM(CASE WHEN runs.triggered_at >= {} AND runs.status = 'failure'::status THEN 1 ELSE 0 END), 0) AS BIGINT)", last_24h_ms)), "current_errors")
        .column_as(Expr::cust(format!("CAST(COALESCE(SUM(CASE WHEN runs.triggered_at < {} AND runs.status = 'failure'::status THEN 1 ELSE 0 END), 0) AS BIGINT)", last_24h_ms)), "prev_errors")

        .column_as(Expr::cust(format!("CAST(AVG(CASE WHEN runs.triggered_at >= {} THEN runs.finish_time - runs.started_at ELSE NULL END) AS DOUBLE PRECISION)", last_24h_ms)), "current_latency")
        .column_as(Expr::cust(format!("CAST(AVG(CASE WHEN runs.triggered_at < {} THEN runs.finish_time - runs.started_at ELSE NULL END) AS DOUBLE PRECISION)", last_24h_ms)), "prev_latency")

        .join(JoinType::InnerJoin, run::Relation::Function.def())
        .filter(function::Column::OwnerId.eq(user_id))
        .filter(run::Column::TriggeredAt.gte(prev_24h_ms))
        .into_model::<RawDashboardMetrics>()
        .one(db)
        .await.context("could not get raw runs metrics")?;

    fn round_2(val: f64) -> f64 {
        (val * 100.0).round() / 100.0
    }

    let stats = if let Some(raw) = raw_metrics {
        let inv_change = if raw.prev_invocations == 0 {
            if raw.current_invocations > 0 {
                100.0
            } else {
                0.0
            }
        } else {
            ((raw.current_invocations - raw.prev_invocations) as f64 / raw.prev_invocations as f64)
                * 100.0
        };

        let current_err_rate = if raw.current_invocations == 0 {
            0.0
        } else {
            (raw.current_errors as f64 / raw.current_invocations as f64) * 100.0
        };

        let prev_err_rate = if raw.prev_invocations == 0 {
            0.0
        } else {
            (raw.prev_errors as f64 / raw.prev_invocations as f64) * 100.0
        };

        let current_lat = raw.current_latency.unwrap_or(0.0);
        let prev_lat = raw.prev_latency.unwrap_or(0.0);

        DashboardStats {
            total_invocations: raw.current_invocations,
            invocations_change_pct: round_2(inv_change),

            error_rate_pct: round_2(current_err_rate),
            error_rate_change_pct: round_2(current_err_rate - prev_err_rate),

            avg_latency_ms: round_2(current_lat),
            latency_change_ms: round_2(current_lat - prev_lat),
        }
    } else {
        DashboardStats {
            total_invocations: 0,
            invocations_change_pct: 0.0,
            error_rate_pct: 0.0,
            error_rate_change_pct: 0.0,
            avg_latency_ms: 0.0,
            latency_change_ms: 0.0,
        }
    };

    Ok(stats)
}

async fn fetch_graph_data(
    user_id: i32,
    db: &DatabaseConnection,
) -> Result<Vec<UsageOverTime>, AppError> {
    let day_bucket_expr = Expr::cust("(runs.triggered_at / 86400000) * 86400000");

    let cumulative_sum_expr = Expr::cust(
        "CAST(SUM(COUNT(runs.id)) OVER (ORDER BY (runs.triggered_at / 86400000) * 86400000 ASC) AS BIGINT)",
    );

    let usage_data: Vec<UsageOverTime> = run::Entity::find()
        .select_only()
        .column_as(day_bucket_expr.clone(), "timestamp")
        .column_as(cumulative_sum_expr, "total_invocations")
        .join(JoinType::InnerJoin, run::Relation::Function.def())
        .filter(function::Column::OwnerId.eq(user_id))
        .group_by(day_bucket_expr.clone())
        .order_by_asc(day_bucket_expr)
        .into_model::<UsageOverTime>()
        .all(db)
        .await
        .context("could not get usage data")?;

    Ok(usage_data)
}

async fn main_page_stats(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
) -> Result<Json<StatsResponse>, AppError> {
   

    let (top_invocations, usage_data, stats) = try_join!(
        fetch_invocations_list(user.id, &state.db),
        fetch_graph_data(user.id, &state.db),
        fetch_metrics(user.id, &state.db)
    )?;

    Ok(Json(StatsResponse {
        invocations: top_invocations.into_iter().map(Into::into).collect(),
        usage_over_time: usage_data.into_iter().map(Into::into).collect(),
        metrics: stats,
    }))
}
