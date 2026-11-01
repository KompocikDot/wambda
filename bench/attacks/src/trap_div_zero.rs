use http::HeaderMap;

// Division by zero: the guest dies, the platform must stay up and report a failure.
pub async fn run(_query: &str, _headers: &HeaderMap) -> (u16, String) {
    let result = 100 / "0".parse::<i32>().unwrap();
    return (crate::FAIL, format!("FAIL: 100 / 0 = {result}"))
}
