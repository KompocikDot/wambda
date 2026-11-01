use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, body::Incoming, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn service(
    req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, std::convert::Infallible> {
    let r = core_handler::handle(req.uri().path());
    Ok(Response::builder()
        .status(r.status)
        .body(Full::new(Bytes::from(r.body)))
        .unwrap())
}

#[tokio::main]
async fn main() {
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let _ = hyper::server::conn::http1::Builder::new()
                .serve_connection(TokioIo::new(stream), service_fn(service))
                .await;
        });
    }
}
