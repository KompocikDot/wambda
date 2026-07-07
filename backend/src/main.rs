use std::sync::Arc;

use anyhow::{Context, Result};
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json, Router,
    extract::{Multipart, State},
    http::StatusCode,
    routing::post,
};

use axum_valid::Valid;
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;

mod convert;
mod errors;
mod executor;
mod models;
mod schemas;
mod state;

use crate::{
    convert::{WasmtimeVal, from_json},
    errors::AppError,
    models::user,
    schemas::user::RegisterUserResponse,
};
use executor::execute_wasm;
use schemas::user::RegisterUserPayload;
use state::AppState;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use uuid::Uuid;

#[derive(Deserialize)]
struct RunPayload {
    binary_id: String,
    fn_args: Vec<WasmtimeVal>,
}

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

    let app = Router::new()
        .route("/register", post(register_user))
        .route("/run", post(run))
        .route("/save", post(create_function))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .context("could not bind tcp listener")?;

    axum::serve(listener, app)
        .await
        .context("could not start http server")?;

    return Ok(());
}

async fn register_user(
    State(state): State<Arc<AppState>>,
    Valid(Json(payload)): Valid<Json<RegisterUserPayload>>,
) -> Result<(StatusCode, Json<RegisterUserResponse>), AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .context("Failed to create hash")?
        .to_string();

    let new_user = user::ActiveModel {
        full_name: Set(payload.full_name),
        email: Set(payload.email),
        password: Set(password_hash),
        ..Default::default()
    };

    let new_id = new_user.insert(&state.db).await.unwrap().id;

    return Ok((
        StatusCode::CREATED,
        Json(RegisterUserResponse { id: new_id }),
    ));
}

async fn create_function(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> StatusCode {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("wasm") {
            let data = field.bytes().await.unwrap();
            let blob_uuid = Uuid::now_v7().to_string();

            let insert = state
                .blob_bucket
                .put_object(&blob_uuid, &data.to_vec())
                .await;

            return match insert {
                Ok(_) => StatusCode::CREATED,
                Err(err) => {
                    eprintln!("{err}");
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            };
        }
    }

    return StatusCode::BAD_REQUEST;
}

async fn run(State(state): State<Arc<AppState>>, Json(payload): Json<RunPayload>) -> &'static str {
    // let start_time = ActiveValue::Set(Utc::now().into());
    let binary = state
        .blob_bucket
        .get_object(payload.binary_id)
        .await
        .unwrap();

    let mut args = Vec::new();
    for arg in payload.fn_args {
        args.push(from_json(arg));
    }

    let out = execute_wasm(binary.as_slice(), args.as_slice()).unwrap();
    println!("{:?}", out);
    // let finish_time = ActiveValue::Set(Utc::now().into());

    // let new_run = runs::ActiveModel {
    //     start_time,
    //     finish_time,
    //     ..Default::default()
    // };

    // new_run.insert(&state.db).await.unwrap();
    "hi, world!"
}
