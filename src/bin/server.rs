use chrono::Local;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub sender: String,
    pub content: String,
    pub avatar: Option<String>,
    pub addr: Option<String>,
    pub timestamp: Option<String>,
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            // Deserialize incoming JSON message
                            if let Ok(mut chat_msg) = serde_json::from_str::<ChatMessage>(text) {
                                // Enriched message with sender's address and timestamp
                                chat_msg.addr = Some(addr.to_string());
                                chat_msg.timestamp = Some(Local::now().format("%H:%M:%S").to_string());
                                
                                // Re-serialize and broadcast
                                if let Ok(serialized) = serde_json::to_string(&chat_msg) {
                                    let _ = bcast_tx.send(serialized);
                                }
                            }
                        }
                    }
                    _ => break,
                }
            }
            msg = bcast_rx.recv() => {
                match msg {
                    Ok(text) => {
                        ws_stream.send(Message::text(text)).await?;
                    }
                    _ => break,
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    // Tutorial 3 web client defaults to ws://127.0.0.1:9001
    let listener = TcpListener::bind("127.0.0.1:9001").await?;
    println!("listening on port 9001");

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
