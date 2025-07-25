use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = url::Url::parse("ws://127.0.0.1:8081")?;
    let (mut ws_stream, _) = connect_async(url).await?;

    ws_stream.send(Message::Text("Hello WebSocket!".to_string())).await?;
    if let Some(Ok(msg)) = ws_stream.next().await {
        println!("Réponse : {}", msg);
    }

    Ok(())
}
