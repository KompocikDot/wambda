use std::env;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::Router;
use axum::routing::get;
use axum::http::{HeaderValue, Method, header};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

mod errors;
mod exctractors;
pub mod executor;
mod handlers;
mod models;
mod schemas;
mod state;

use tower_http::cors::CorsLayer;
use tower_sessions_cookie_store::{CookieSessionConfig, CookieSessionManagerLayer, Key};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::handlers::auth::auth_router;
use crate::handlers::executor::executor_router;
use crate::handlers::function::functions_router;
use crate::handlers::run::runs_router;
use crate::handlers::stats::stats_router;
use crate::handlers::user::user_router;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=info,tower_http=info,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv::dotenv().ok();

    let state = Arc::new(
        AppState::new()
            .await
            .context("Could not create pool connection to the postgres database")?,
    );

    println!("migrating db");
    state
        .db
        .get_schema_registry(module_path!().split("::").next().unwrap())
        .sync(&state.db)
        .await
        .context("could not migrate db")?;

    let session_config = CookieSessionConfig::default().with_secure(!state.debug_mode);

    let session_key = env::var("BACKEND_SESSION_COOKIE_SECRET")?;
    let secret = Key::from(&BASE64.decode(session_key)?);

    let cors_origin = env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".to_owned())
        .parse::<HeaderValue>()
        .context("invalid CORS_ORIGIN")?;
    let addr = env::var("BACKEND_ADDR").unwrap_or_else(|_| "127.0.0.1:8000".to_owned());

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .nest("/api/auth", auth_router())
        .nest("/api/user", user_router())
        .nest("/api/functions", functions_router())
        .nest("/api/execute", executor_router())
        .nest("/api/runs", runs_router())
        .nest("/api/stats", stats_router())
        .with_state(state)
        .layer(CookieSessionManagerLayer::signed(secret).with_config(session_config))
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_origin(cors_origin)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::PUT,
                    Method::DELETE,
                ])
                .allow_headers([header::CONTENT_TYPE]),
        );

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("could not bind tcp listener")?;

    println!("starting server...");

    axum::serve(listener, app)
        .await
        .context("could not start http server")?;

    return Ok(());
}
