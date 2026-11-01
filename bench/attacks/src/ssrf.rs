use bytes::Bytes;
use http::HeaderMap;
use http_body_util::Empty;
use wasip3::http::{client, types::ErrorCode};
use wasip3::http_compat::http_into_wasi_request;

// Sends a request with url set to url passed as query param.
// allow=1 means that request should pass (it's safe),
// allow=0 means it should fail with DestinationIpProhibited error.
pub async fn run(query: &str, _headers: &HeaderMap) -> (u16, String) {
    let allow = query.split('&').any(|p| p == "allow=1");
    let url = query
        .split('&')
        .find_map(|p| p.strip_prefix("url="))
        .unwrap_or("");

    let request = match http::Request::get(url).body(Empty::<Bytes>::new()) {
        Ok(request) => request,
        Err(e) => return (500, format!("bad url: {e}")),
    };

    let wasi_request = match http_into_wasi_request(request) {
        Ok(request) => request,
        Err(e) => return (500, format!("bad request: {e:?}")),
    };

    let (reached, msg) = match client::send(wasi_request).await {
        Err(ErrorCode::DestinationIpProhibited) => {
            (false, "blocked: destination IP prohibited".to_string())
        }
        Err(e) => (true, format!("request left the sandbox, error: {e:?}")),
        Ok(response) => (
            true,
            format!("got response, status {}", response.get_status_code()),
        ),
    };

    let status = if reached == allow {
        crate::PASS
    } else {
        crate::FAIL
    };
    return (status, msg);
}
