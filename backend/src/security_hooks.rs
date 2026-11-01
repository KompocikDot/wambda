use std::net::IpAddr;

use wasmtime_wasi::TrappableError;
use wasmtime_wasi_http::p3::bindings::http::types::{DnsErrorPayload, ErrorCode};
use wasmtime_wasi_http::p3::{RequestOptions, WasiHttpHooks, default_send_request};

use axum::body::Bytes;
use http_body_util::{BodyExt, combinators::UnsyncBoxBody};
use ipnet::IpNet;


/// https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html#deny-list-last-resort
const BLOCKED: &[&str] = &[
    "100.64.0.0/10",
    "198.18.0.0/15",
    "240.0.0.0/4",
    "fec0::/10",
    "2001:db8::/32",
    "64:ff9b::/96",
    "2002::/16",
];

fn allowed_address_range(addr: IpAddr) -> bool {
    let addr = addr.to_canonical();
    if BLOCKED
        .iter()
        .any(|n| n.parse::<IpNet>().unwrap().contains(&addr))
    {
        return false;
    }

    match addr {
        IpAddr::V4(addr) => {
            if addr.is_loopback()
                || addr.is_link_local()
                || addr.is_multicast()
                || addr.is_unspecified()
                || addr.is_documentation()
                || addr.is_private()
                || addr.is_broadcast()
            {
                return false;
            }
        }
        IpAddr::V6(addr) => {
            if addr.is_loopback()
                || addr.is_unspecified()
                || addr.is_multicast()
                || addr.is_unique_local()
                || addr.is_unicast_link_local()
            {
                return false;
            }
        }
    }

    return true;
}

#[derive(Default)]
pub struct HttpSecurityHook;

impl WasiHttpHooks for HttpSecurityHook {
    fn send_request(
        &mut self,
        request: http::Request<UnsyncBoxBody<Bytes, ErrorCode>>,
        options: Option<RequestOptions>,
        _fut: Box<dyn Future<Output = Result<(), ErrorCode>> + Send>,
    ) -> Box<
        dyn Future<
                Output = Result<
                    (
                        http::Response<UnsyncBoxBody<Bytes, ErrorCode>>,
                        Box<dyn Future<Output = Result<(), ErrorCode>> + Send>,
                    ),
                    TrappableError<ErrorCode>,
                >,
            > + Send,
    > {
        Box::new(async move {
            let host = request.uri().host().unwrap_or_default().to_string();
            let default_port = match request.uri().scheme_str() {
                Some("http") => 80,
                _ => 443,
            };
            let port = request.uri().port_u16().unwrap_or(default_port);

            let resolved_addresses = tokio::net::lookup_host((host.as_str(), port))
                .await
                .map_err(|_| {
                    ErrorCode::DnsError(DnsErrorPayload {
                        rcode: None,
                        info_code: None,
                    })
                })?;

            for addr in resolved_addresses {
                if !allowed_address_range(addr.ip()) {
                    return Err(ErrorCode::DestinationIpProhibited.into());
                }
            }

            let (res, io) = default_send_request(request, options).await?;
            Ok((
                res.map(BodyExt::boxed_unsync),
                Box::new(io) as Box<dyn Future<Output = _> + Send>,
            ))
        })
    }
}
