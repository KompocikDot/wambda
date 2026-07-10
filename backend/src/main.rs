use std::env;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::Router;
use axum::http::{HeaderValue, Method, header};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

mod errors;
mod exctractors;
mod handlers;
mod models;
mod schemas;
mod state;

use state::AppState;
use tower_http::cors::CorsLayer;
use tower_sessions_cookie_store::{CookieSessionConfig, CookieSessionManagerLayer, Key};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::handlers::auth::auth_router;
use crate::handlers::function::functions_router;
use crate::handlers::user::user_router;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv::dotenv().context("could not load .env file")?;

    let state = Arc::new(
        AppState::new()
            .await
            .context("Could not create pool connection to the postgres database")?,
    );

    state
        .db
        .get_schema_registry(module_path!().split("::").next().unwrap())
        .sync(&state.db)
        .await
        .context("could not migrate db")?;

    let session_config = CookieSessionConfig::default().with_secure(!state.debug_mode);

    let session_key = env::var("BACKEND_SESSION_COOKIE_SECRET")?;
    let secret = Key::from(&BASE64.decode(session_key)?);

    let app = Router::new()
        .nest("/api/auth", auth_router())
        .nest("/api/user", user_router())
        .nest("/api/functions", functions_router())
        .with_state(state)
        .layer(CookieSessionManagerLayer::signed(secret).with_config(session_config))
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::PUT,
                    Method::DELETE,
                ])
                .allow_headers([header::CONTENT_TYPE]),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .context("could not bind tcp listener")?;

    axum::serve(listener, app)
        .await
        .context("could not start http server")?;

    return Ok(());
}
