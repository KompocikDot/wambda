use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::body::{Body as AxumBody, Bytes};
use axum::extract::Request;
use axum::response::Response;
use http_body_util::BodyExt;
use wasmtime::component::{Accessor, Component, Linker, ResourceTable};
use wasmtime::{Engine, Store};
use wasmtime_wasi::cli::OutputFile;
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_http::p3::bindings::Service;
use wasmtime_wasi_http::p3::bindings::http::types::ErrorCode;
use wasmtime_wasi_http::p3::{Request as WasiRequest, WasiHttpView};
use wasmtime_wasi_http::{WasiHttpCtx, p3::WasiHttpCtxView};

pub struct ExecutionParams {
    stdout_path: String,
    stderr_path: String,
    envs: HashMap<String, String>,
}

impl ExecutionParams {
    pub fn new(stdout_path: String, stderr_path: String, envs: HashMap<String, String>) -> Self {
        Self {
            stdout_path,
            stderr_path,
            envs,
        }
    }
}

#[derive(Default)]
struct WASMState {
    table: ResourceTable,
    http_ctx: WasiHttpCtx,
    cli_ctx: WasiCtx,
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

// Calls the guest's `handle` export and turns the wasi `Response` resource
// into a plain, fully-buffered `http::Response`.
//
// Returns `Ok(Err(code))` for a wasi-level error-code (e.g. guest rejected the
// request); returns `Err(_)` only for host-side failures (traps, wasmtime errors).
async fn handle_and_collect(
    accessor: &Accessor<WASMState>,
    service: &Service,
    req: WasiRequest,
) -> Result<Result<http::Response<Bytes>, ErrorCode>> {
    // Invoke the guest's exported `handle` function with our request resource.
    let res = match service.handle(accessor, req).await? {
        Ok(res) => res,
        Err(code) => return Ok(Err(code)), // guest returned an http error-code, not a trap
    };
    // Convert the wasi `Response` resource into an `http::Response` with a
    // streaming body; `store` is borrowed briefly via `accessor.with`.
    let res = accessor.with(|store| res.into_http(store, async { Ok(()) }))?;
    let (parts, body) = res.into_parts();
    // Drain the streaming body into memory so we can hand back a plain response.
    let bytes = body
        .collect()
        .await
        .context("failed to collect response body")?
        .to_bytes();
    Ok(Ok(http::Response::from_parts(parts, bytes)))
}

/// Loads a wasm component from `binary`, instantiates it against the
/// `wasi:http/service` world, forwards `axum_req` to its `handle` export, and
/// converts the guest's response back into an axum `Response`.
pub async fn execute_wasm(
    engine: Arc<Engine>,
    axum_req: Request,
    binary: &[u8],
    params: ExecutionParams,
) -> Result<Response> {
    // Component-model async (needed for wasi:http/wasi:cli p3, which use
    // `async func`/streams/futures instead of poll-based p2 APIs).

    // Wire up the host-side implementations of wasi:http and wasi:cli (p3)
    // so the guest can import them.
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

    // Fresh per-request host state (resource table + wasi contexts).

    let state = WASMState {
        cli_ctx: WasiCtx::builder()
            .stdout(OutputFile::new(stdout))
            .stderr(OutputFile::new(stderr))
            .envs(envs.as_ref())
            .build(),
        http_ctx: WasiHttpCtx::default(),
        table: ResourceTable::default(),
    };

    let mut store = Store::new(&engine, state);

    // WARNING: Be careful, this code have to be deserialzed on a same version as it was compiled.
    let component = unsafe { Component::deserialize(&engine, binary)? };

    // The guest must export `wasi:http/handler@0.3.0` (part of the
    // `wasi:http/service` world): `handle: async func(request) -> result<response, error-code>`.
    let service = Service::instantiate_async(&mut store, &component, &linker).await?;

    // Convert the incoming axum request into a wasi `Request` value.
    // `axum::Error` doesn't know how to become a wasi `error-code`, so map it first.
    let (parts, body) = axum_req.into_parts();
    let body = body.map_err(|err| ErrorCode::InternalError(Some(err.to_string())));
    // `io` is a future that drives streaming the request body into the guest;
    // it must be polled concurrently alongside the `handle` call below.
    let (req, io) = WasiRequest::from_http(http::Request::from_parts(parts, body));

    // `run_concurrent` gives us an `Accessor` for calling async component
    // exports. We race the `handle` call against the body-streaming future so
    // both make progress together.
    let res = store
        .run_concurrent(async |accessor| {
            let (res, ()) = tokio::try_join!(handle_and_collect(accessor, &service, req), async {
                io.await.context("failed to stream request body into guest")
            })?;
            anyhow::Ok(res)
        })
        .await??; // first `?` = wasmtime::Result, second `?` = our anyhow::Result

    // Turn the collected wasi response into a real axum response, or bail if
    // the guest reported an http-level error.
    match res {
        Ok(response) => {
            let (parts, body) = response.into_parts();
            Ok(Response::from_parts(parts, AxumBody::from(body)))
        }
        Err(code) => anyhow::bail!("guest handler returned an http error: {code:?}"),
    }
}
