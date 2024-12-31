use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle(mut socket: tokio::net::TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => return,
            Ok(n) => {
                if socket.write_all(&buffer[0..n]).await.is_err() {
                    return;
                }
            }
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
