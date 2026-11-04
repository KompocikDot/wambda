pub struct Reply {
    pub status: u16,
    pub body: String,
}

// Multiplies two n x n matrices with the naive triple loop and returns the sum of the result.
// The inputs are small whole numbers, so the f64 sum is exact and every variant must return the same value.
fn matmul(n: usize) -> f64 {
    let a: Vec<f64> = (0..n * n).map(|i| ((i / n + i % n) % 7) as f64).collect();
    let b: Vec<f64> = (0..n * n).map(|i| ((i / n * 2 + i % n) % 5) as f64).collect();
    let mut c = vec![0.0; n * n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i * n + j] += a[i * n + k] * b[k * n + j];
            }
        }
    }

    c.iter().sum()
}

pub fn handle(path: &str) -> Reply {
    let (status, body) = match path
        .trim_start_matches('/')
        .split('/')
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["hello"] => (200, "hello".to_string()),
        ["matmul", n] => match n.parse::<usize>() {
            Ok(n) if n <= 2000 => (200, matmul(n).to_string()),
            _ => (400, "bad n".to_string()),
        },
        _ => (404, "not found".to_string()),
    };
    
    return Reply { status, body };
}
