use wasip3::http::types::{ErrorCode, Request, Response};
use wasip3::http_compat::{http_from_wasi_request, http_into_wasi_response};

wasip3::http::service::export!(Example);

struct Example;

impl wasip3::exports::http::handler::Guest for Example {
    async fn handle(request: Request) -> Result<Response, ErrorCode> {
        let request = http_from_wasi_request(request)?;
        let r = core_handler::handle(request.uri().path());
        let response = http::Response::builder()
            .status(r.status)
            .body(r.body)
            .map_err(|e| ErrorCode::InternalError(Some(e.to_string())))?;
        http_into_wasi_response(response)
    }
}
