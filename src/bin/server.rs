use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::accept_async;
use tokio::net::TcpListener;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    println!("Serveur WebSocket sur ws://127.0.0.1:8081");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = ws_stream.split();

            while let Some(Ok(msg)) = read.next().await {
                println!("Reçu: {}", msg);
                if let Message::Text(text) = msg {
                    let reply = Message::Text(format!("Echo: {}", text));
                    write.send(reply).await.unwrap();
                }
            }
        });
    }

    Ok(())
}
