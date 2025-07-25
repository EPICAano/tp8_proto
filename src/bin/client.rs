use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use bincode;
mod common;
use common::Message;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9000").await?;

    let msg = Message::Hello("Alice".to_string());
    let data = bincode::serialize(&msg).unwrap();
    stream.write_all(&data).await?;

    let mut buf = [0u8; 1024];
    let size = stream.read(&mut buf).await?;
    let response: Message = bincode::deserialize(&buf[..size]).unwrap();

    println!("Réponse du serveur: {:?}", response);
    Ok(())
}
