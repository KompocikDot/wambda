use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{
    Router,
    extract::{Request, State},
    response::Response,
    routing::any,
};
use tokio::fs::{self, File};
use uuid::Uuid;

use crate::{errors::AppError, executor::execute_wasm, state::AppState};

pub fn executor_router() -> Router<Arc<AppState>> {
    return Router::new().route("/", any(execute));
}

async fn execute(State(state): State<Arc<AppState>>, req: Request) -> Result<Response, AppError> {
    let run_id = Uuid::now_v7();

    let stdout_filepath = format!("/tmp/{}:stdout", run_id);
    let stderr_filepath = format!("/tmp/{}:stderr", run_id);

    let mut stdout = File::options()
        .create(true)
        .append(true)
        .read(true)
        .open(&stdout_filepath)
        .await
        .context("could not create wasi stdout")?;

    let mut stderr = File::options()
        .create(true)
        .append(true)
        .read(true)
        .open(&stderr_filepath)
        .await
        .context("could not create wasi stderr")?;

    // TODO: Fix that path to get corrrect binary based on payload
    let binary = state
        .blob_bucket
        .get_object("")
        .await
        .context("could not read wasm binary from bucket")?;

    let resp = execute_wasm(req, binary.as_slice()).await?;

    state
        .io_executions_bucket
        .put_object_stream(&mut stdout, format!("/{}:stdout", run_id))
        .await
        .context("could not write stdout to bucket")?;
    state
        .io_executions_bucket
        .put_object_stream(&mut stderr, format!("/{}:stderr", run_id))
        .await
        .context("could not write stderr to bucket")?;

    fs::remove_file(stdout_filepath)
        .await
        .context("could not remove wasi stdout temp file")?;
    fs::remove_file(stderr_filepath)
        .await
        .context("could not remove wasi stderr temp file")?;

    Ok(resp)
}
