use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::io::AsyncBufReadExt;

use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Request {
    method: String,
    number: f64,
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

async fn handle(socket: tokio::net::TcpStream) {
    let mut reader = BufReader::new(socket);
    let mut buffer = Vec::new();

    loop {
        buffer.clear();
        match reader.read_until(b'\n', &mut buffer).await {
            Ok(0) => return,
            Ok(n) => {
                let request: Result<Request, _> = serde_json::from_slice(&buffer[0..n]);
                let response = match request {
                    Ok(req) if req.method == "isPrime" => {
                        let prime = if req.number < 1. || req.number.trunc() != req.number {
                            false
                        } else {
                            is_prime(req.number as u64)
                        };
                        json!({ "method": "isPrime", "prime": prime })
                    }
                    _ => json!({ "error": "Invalid request" }),
                };

                let mut response = serde_json::to_vec(&response).expect("Failed to serialize response");
                response.push(b'\n');
                if reader.get_mut().write_all(&response).await.is_err() {
                    return;
                }            }
            Err(_) => return,
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").await.expect("Could not bind");
    println!("Server listening on port 7878");

    loop {
        let (socket, _) = listener.accept().await.expect("Failed to accept connection");

        tokio::spawn(handle(socket));
    }
}