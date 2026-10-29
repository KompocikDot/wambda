use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use axum::body::{Body as AxumBody, Bytes};
use axum::extract::Request;
use axum::response::Response;
use http_body_util::BodyExt;
use wasmtime::component::{Accessor, Component, Linker, ResourceTable};
use wasmtime::{Engine, Store, StoreLimits, StoreLimitsBuilder};
use wasmtime_wasi::cli::OutputFile;
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_http::p3::bindings::Service;
use wasmtime_wasi_http::p3::bindings::http::types::ErrorCode;
use wasmtime_wasi_http::p3::{Request as WasiRequest, WasiHttpView};
use wasmtime_wasi_http::{WasiHttpCtx, p3::WasiHttpCtxView};

pub struct ExecutionParams {
    stdout_path: String,
    stderr_path: String,
    envs: HashMap<String, String>,
    workdir_path: String,
    memory_bytes: usize,
    timeout: Duration,
}

impl ExecutionParams {
    pub fn new(
        stdout_path: String,
        stderr_path: String,
        workdir_path: String,
        envs: HashMap<String, String>,
        memory_bytes: usize,
        timeout: Duration,
    ) -> Self {
        Self {
            stdout_path,
            stderr_path,
            envs,
            workdir_path,
            memory_bytes,
            timeout,
        }
    }
}

#[derive(Default)]
struct WASMState {
    table: ResourceTable,
    http_ctx: WasiHttpCtx,
    cli_ctx: WasiCtx,
    limits: StoreLimits,
}

impl WasiHttpView for WASMState {
    fn http(&mut self) -> WasiHttpCtxView<'_> {
        WasiHttpCtxView {
            hooks: Default::default(),
            table: &mut self.table,
            ctx: &mut self.http_ctx,
        }
    }
}

impl WasiView for WASMState {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.cli_ctx,
            table: &mut self.table,
        }
    }
}

async fn handle_and_collect(
    accessor: &Accessor<WASMState>,
    service: &Service,
    req: WasiRequest,
) -> Result<Result<http::Response<Bytes>, ErrorCode>> {
    let res = match service.handle(accessor, req).await? {
        Ok(res) => res,
        Err(code) => return Ok(Err(code)),
    };

    let res = accessor.with(|store| res.into_http(store, async { Ok(()) }))?;
    let (parts, body) = res.into_parts();
    let bytes = body
        .collect()
        .await
        .context("failed to collect response body")?
        .to_bytes();
    Ok(Ok(http::Response::from_parts(parts, bytes)))
}

pub async fn execute_wasm(
    engine: Arc<Engine>,
    axum_req: Request,
    binary: &[u8],
    params: ExecutionParams,
) -> Result<Response> {
    let mut linker = Linker::<WASMState>::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
    wasmtime_wasi::p3::add_to_linker(&mut linker)?;
    wasmtime_wasi_http::p3::add_to_linker(&mut linker)?;

    let stdout = File::options()
        .append(true)
        .open(params.stdout_path)
        .context("could not create wasi stdout")?;

    let stderr = File::options()
        .append(true)
        .open(params.stderr_path)
        .context("could not create wasi stderr")?;

    let envs = params.envs.iter().collect::<Vec<(&String, &String)>>();

    let state = WASMState {
        cli_ctx: WasiCtx::builder()
            .stdout(OutputFile::new(stdout))
            .stderr(OutputFile::new(stderr))
            .envs(envs.as_ref())
            .preopened_dir(params.workdir_path, "/tmp", DirPerms::all(), FilePerms::all())?
            .build(),
        http_ctx: WasiHttpCtx::default(),
        table: ResourceTable::default(),
        limits: StoreLimitsBuilder::new()
            .memory_size(params.memory_bytes)
            .trap_on_grow_failure(true)
            .build(),
    };

    let mut store = Store::new(&engine, state);
    store.limiter(|state| &mut state.limits);
    store.epoch_deadline_async_yield_and_update(1);

    let component = unsafe { Component::deserialize(&engine, binary)? };

    let service = Service::instantiate_async(&mut store, &component, &linker).await?;

    let (parts, body) = axum_req.into_parts();
    let body = body.map_err(|err| ErrorCode::InternalError(Some(err.to_string())));
    let (req, io) = WasiRequest::from_http(http::Request::from_parts(parts, body));

    let run = store.run_concurrent(async |accessor| {
        let (res, ()) = tokio::try_join!(handle_and_collect(accessor, &service, req), async {
            io.await.context("failed to stream request body into guest")
        })?;
        anyhow::Ok(res)
    });
    let res = tokio::time::timeout(params.timeout, run)
        .await
        .context("function execution timed out")???;

    match res {
        Ok(response) => {
            let (parts, body) = response.into_parts();
            Ok(Response::from_parts(parts, AxumBody::from(body)))
        }
        Err(code) => anyhow::bail!("guest handler returned an http error: {code:?}"),
    }
}
