use std::sync::Arc;

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::Mutex,
};

#[derive(Debug)]
struct State {
    id: usize,
    name: String,
    socket: tokio::io::WriteHalf<tokio::net::TcpStream>,
}

async fn handle(
    socket: tokio::net::TcpStream,
    connection_id: usize,
    connections: Arc<Mutex<Vec<State>>>,
) {
    let mut buffer_name = vec![0; 32];
    let (mut read, mut write) = tokio::io::split(socket);

    write
        .write_all("Welcome to budgetchat! What shall I call you?\n".as_bytes())
        .await
        .expect("message to be sent");
    let name_len = read.read(&mut buffer_name).await.expect("an username");

    let name = match std::str::from_utf8(&buffer_name[..name_len]) {
        Ok(name) => name.trim().to_string(),
        Err(_) => {
            write
                .write_all("Invalid username\n".as_bytes())
                .await
                .expect("error message to be sent");
            return;
        }
    };

    if name.chars().any(|c| !c.is_alphanumeric()) || name.is_empty() {
        return;
    }

    {
        let mut connections = connections.lock().await;
        let mut names: Vec<String> = Vec::new();

        for conn in connections.iter_mut() {
            conn.socket
                .write_all(format!("* {} has entered the room\n", name).as_bytes())
                .await
                .unwrap();
            names.push(conn.name.clone());
        }

        let names: Vec<String> = connections
            .iter()
            .map(|s| s.name.clone())
            .collect::<Vec<String>>();
        write
            .write_all(format!("* The room contains: {}\n", names.join(", ")).as_bytes())
            .await
            .unwrap();

        connections.push(State {
            id: connection_id,
            socket: write,
            name: name.clone(),
        });
    }

    let mut reader = BufReader::new(read);
    let mut buffer = vec![0; 2028];

    loop {
        buffer.clear();
        match reader.read_until(b'\n', &mut buffer).await {
            Ok(0) => {
                let mut connections = connections.lock().await;
                let state_index = connections
                    .iter()
                    .position(|x| x.id == connection_id)
                    .unwrap();
                connections.remove(state_index);

                for conn in connections.iter_mut() {
                    conn.socket
                        .write_all(format!("* {} has left the room\n", name).as_bytes())
                        .await
                        .unwrap();
                }
                return;
            }
            Ok(_) => {
                let message = buffer.clone();
                let mut connections = connections.lock().await;
                for conn in connections.iter_mut() {
                    if conn.id == connection_id {
                        continue;
                    }

                    conn.socket
                        .write_all(
                            format!("[{}] {}", name, std::str::from_utf8(&message).unwrap())
                                .as_bytes(),
                        )
                        .await
                        .unwrap();
                }
                //write.write_all(&message).await.unwrap();
            }
            Err(_) => return,
        }
    }
}

#[tokio::main]
async fn main() {
    let connections = Arc::new(Mutex::new(Vec::new()));

    let listener = TcpListener::bind("0.0.0.0:7878")
        .await
        .expect("Could not bind");
    println!("Server listening on port 7878");

    let mut connection_id = 0;

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let connections = Arc::clone(&connections);

        tokio::spawn(handle(socket, connection_id, connections));

        connection_id += 1;
    }
}
