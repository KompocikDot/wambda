pub struct Reply {
    pub status: u16,
    pub body: String,
}

fn fib(n: u32) -> u64 {
    if n < 2 {
        return n as u64;
    }

    return fib(n - 1) + fib(n - 2);
}

pub fn handle(path: &str) -> Reply {
    let (status, body) = match path
        .trim_start_matches('/')
        .split('/')
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["hello"] => (200, "hello".to_string()),
        ["fib", n] => match n.parse::<u32>() {
            Ok(n) if n <= 50 => (200, fib(n).to_string()),
            _ => (400, "bad n".to_string()),
        },
        _ => (404, "not found".to_string()),
    };
    
    return Reply { status, body };
}
