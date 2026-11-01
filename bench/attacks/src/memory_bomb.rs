use std::hint::black_box;

use http::HeaderMap;

pub async fn run(_query: &str, _headers: &HeaderMap) -> (u16, String) {
    let mut chunks: Vec<Vec<u8>> = Vec::new();
    // allocate 1Gb in 1Mb chunks
    for _ in 0..1024 {
        chunks.push(vec![1 as u8; 1 << 20]);
    }
    black_box(&chunks);

    return (crate::FAIL, format!("FAIL: allocated {} MB", chunks.len()));
}
