use std::hint::black_box;
use http::HeaderMap;

pub async fn run(_query: &str, _headers: &HeaderMap) -> (u16, String) {
    let mut counter: i64 = 0;
    loop {
        counter = black_box(counter.wrapping_add(1));
    }
}
