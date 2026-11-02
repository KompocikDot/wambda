use http::HeaderMap;

const SECRET_MARKERS: [&str; 6] = [
    "DATABASE", "POSTGRES", "RUSTFS", "SECRET", "PASSWORD", "BACKEND_",
];

pub async fn run(_query: &str, _headers: &HeaderMap) -> (u16, String) {
    for var in std::env::vars() {
        let key = var.0.as_str();
        if SECRET_MARKERS.iter().any(|m| key.contains(m)) {
            return (crate::FAIL, String::from(""));
        }
    }

    return (crate::PASS, String::from(""));
}
