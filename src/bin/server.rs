use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bincode;
mod common;
use common::Message;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    println!("Serveur TCP en écoute sur 127.0.0.1:9000");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let size = socket.read(&mut buf).await.unwrap();
            let msg: Message = bincode::deserialize(&buf[..size]).unwrap();

            println!("Reçu: {:?}", msg);

            let response = match msg {
                Message::Hello(name) => Message::Hello(format!("Salut, {name}!")),
                Message::Bye => Message::Bye,
            };

            let reply = bincode::serialize(&response).unwrap();
            socket.write_all(&reply).await.unwrap();
        });
    }
}
