use http::HeaderMap;
use std::fs;

// Try writing/reading to/from allowed and disallowed files/dirs.
pub async fn run(_query: &str, _headers: &HeaderMap) -> (u16, String) {
    if fs::write("/tmp/ok.txt", "x")
        .and_then(|_| fs::read_to_string("/tmp/ok.txt"))
        .is_err()
    {
        return (500, "control failed: /tmp is not usable".to_string());
    }

    for path in [
        "/etc/passwd",
        "/proc/self/environ",
        "/tmp/../../../etc/passwd",
        "../../../etc/passwd",
        "/",
    ] {
        if fs::read(path).is_ok() || fs::read_dir(path).is_ok() {
            return (crate::FAIL, format!("unexpected read: {path}"));
        }
    }

    for path in ["/escape.txt", "/tmp/../escape.txt"] {
        if fs::write(path, "x").is_ok() {
            return (crate::FAIL, format!("unexpected write: {path}"));
        }
    }

    (crate::PASS, "no unexpected access".to_string())
}
