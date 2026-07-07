use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Multipart, State},
    http::StatusCode,
    routing::post,
};

use serde::Deserialize;

mod convert;
mod executor;
mod models;
mod state;

use crate::convert::{WasmtimeVal, from_json};
use executor::execute_wasm;
use state::AppState;
use uuid::Uuid;

#[derive(Deserialize)]
struct RunPayload {
    binary_id: String,
    fn_args: Vec<WasmtimeVal>,
}

#[tokio::main]
async fn main() {
    match dotenv::dotenv() {
        Ok(_) => (),
        Err(error) => panic!("Could not read .env file: {error:?}"),
    }

    let state = match AppState::new().await {
        Ok(state) => Arc::new(state),
        Err(error) => {
            panic!("Could not create pool connection to the postgres database: {error:?}")
        }
    };

    state
        .db
        .get_schema_registry(module_path!().split("::").next().unwrap())
        .sync(&state.db)
        .await
        .unwrap();

    let app = Router::new()
        .route("/run", post(run))
        .route("/save", post(create_function))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
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
