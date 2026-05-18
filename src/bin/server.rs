use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Membuat receiver baru untuk berlangganan ke saluran siaran server
    let mut bcast_rx = bcast_tx.subscribe();

    // Mengirim pesan sambutan awal bawaan modul praktikum
    ws_stream.send(Message::text("Sean's Computer From server: Welcome to chat! Type a message")).await?;

    loop {
        tokio::select! {
            // Menerima pesan teks dari WebSocket klien ini
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            // KODE SEBELUM PERUBAHAN:
                            // Langsung meneruskan teks mentah asli dari klien ke saluran broadcast
                            let _ = bcast_tx.send(text.to_string());
                        }
                    }
                    _ => break,
                }
            }
            // Menerima pesan kiriman dari saluran broadcast utama peladen
            msg = bcast_rx.recv() => {
                match msg {
                    Ok(text) => {
                        // Meneruskan pesan siaran bersih ke terminal klien ini
                        ws_stream.send(Message::text(format!("Sean's Computer From server: {}", text))).await?;
                    }
                    _ => break,
                }
            }
        }
    }
    Ok(())
}

// ... (fungsi handle_connection tetap sama seperti Experiment 2.1) ...

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();

        tokio::spawn(async move {
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await.unwrap();
            let _ = handle_connection(addr, ws_stream, bcast_tx).await;
        });
    }
}