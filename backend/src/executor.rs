use anyhow::{Context, Result};
use axum::body::{Body as AxumBody, Bytes};
use axum::extract::Request;
use axum::response::Response;
use http_body_util::BodyExt;
use tokio::fs::File;
use wasmtime::component::{Accessor, Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_http::p3::bindings::Service;
use wasmtime_wasi_http::p3::bindings::http::types::ErrorCode;
use wasmtime_wasi_http::p3::{Request as WasiRequest, WasiHttpView};
use wasmtime_wasi_http::{WasiHttpCtx, p3::WasiHttpCtxView};

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

pub async fn execute_wasm(axum_req: Request, binary: &[u8]) -> Result<Response> {
    let mut config = Config::new();
    config.wasm_component_model_async(true);

    let engine = Engine::new(&config)?;

    let mut linker = Linker::<WASMState>::new(&engine);
    wasmtime_wasi_http::p3::add_to_linker(&mut linker)?;
    wasmtime_wasi::p3::add_to_linker(&mut linker)?;

    let state = WASMState {
        cli_ctx: WasiCtx::builder().build(),
        http_ctx: WasiHttpCtx::default(),
        table: ResourceTable::default(),
    };

    let mut store = Store::new(&engine, state);
    let component =
        Component::from_binary(&engine, binary).unwrap(); //.context("supplied binary is not a valid wasm component")?;

    // The guest must export `wasi:http/handler@0.3.0` (part of the
    // `wasi:http/service` world): `handle: async func(request) -> result<response, error-code>`.
    let service = Service::instantiate_async(&mut store, &component, &linker)
        .await.unwrap();
        // .context("failed to instantiate wasi:http/service world")?;

    // `axum::Error` doesn't know how to become a wasi `error-code`, so map it first.
    let (parts, body) = axum_req.into_parts();
    let body = body.map_err(|err| ErrorCode::InternalError(Some(err.to_string())));
    let (req, io) = WasiRequest::from_http(http::Request::from_parts(parts, body));

    let res = store
        .run_concurrent(async |accessor| {
            let (res, ()) = tokio::try_join!(
                handle_and_collect(accessor, &service, req),
                async { io.await.context("failed to stream request body into guest") }
            )?;
            anyhow::Ok(res)
        })
        .await??;

    match res {
        Ok(response) => {
            let (parts, body) = response.into_parts();
            Ok(Response::from_parts(parts, AxumBody::from(body)))
        }
        Err(code) => anyhow::bail!("guest handler returned an http error: {code:?}"),
    }
}