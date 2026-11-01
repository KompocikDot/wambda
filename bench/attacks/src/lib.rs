#![allow(dead_code)]

use wasip3::http::types::{ErrorCode, Request, Response};
use wasip3::http_compat::{http_from_wasi_request, http_into_wasi_response};

mod env_leak;
mod fs_escape;
mod header_leak;
mod infinite_loop;
mod memory_bomb;
mod ssrf;
mod trap_div_zero;

// Each attack is a separate Wasm module (one cargo feature = one module).
#[cfg(feature = "attack-env-leak")]
use env_leak::run;
#[cfg(feature = "attack-fs-escape")]
use fs_escape::run;
#[cfg(feature = "attack-header-leak")]
use header_leak::run;
#[cfg(feature = "attack-infinite-loop")]
use infinite_loop::run;
#[cfg(feature = "attack-memory-bomb")]
use memory_bomb::run;
#[cfg(feature = "attack-ssrf")]
use ssrf::run;
#[cfg(feature = "attack-trap-div-zero")]
use trap_div_zero::run;

pub const PASS: u16 = 200;
pub const FAIL: u16 = 403;

wasip3::http::service::export!(Attack);

struct Attack;

impl wasip3::exports::http::handler::Guest for Attack {
    async fn handle(request: Request) -> Result<Response, ErrorCode> {
        let request = http_from_wasi_request(request)?;
        let query = request.uri().query().unwrap_or("").to_owned();
        let headers = request.headers().clone();
        let (status, body) = run(&query, &headers).await;

        let response = http::Response::builder()
            .status(status)
            .body(body)
            .map_err(|e| ErrorCode::InternalError(Some(e.to_string())))?;

        return http_into_wasi_response(response);
    }
}
