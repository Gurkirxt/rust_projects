use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Chat server running on 127.0.0.1:8080");

    let (tx, _) = broadcast::channel::<String>(100);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Client connected: {}", addr);

        let tx = tx.clone();
        let rx = tx.subscribe();

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, tx, rx).await {
                eprintln!("Error handling client {}: {:?}", addr, e);
            }
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
) -> std::io::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    writer.write_all(b"Enter your name: ").await?;
    reader.read_line(&mut line).await?;
    let name = line.trim().to_string();
    let welcome_message = format!("{} has joined the chat!\n", name);
    tx.send(welcome_message).unwrap();

    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                if result? == 0 {
                    break; // Connection closed
                }
                let message = format!("{}: {}", name, line.trim());
                tx.send(message).unwrap();
                line.clear();
            }
            result = rx.recv() => {
                if let Ok(message) = result {
                    writer.write_all(message.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
            }
        }
    }

    let goodbye_message = format!("{} has left the chat.\n", name);
    tx.send(goodbye_message).unwrap();
    Ok(())
}

