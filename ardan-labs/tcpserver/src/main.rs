use std::time::Duration;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
};

async fn tcp_client() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;
    println!("Connected to the server");
    stream.write_all(b"Hello World").await?;
    let mut buf = vec![0u8; 1024];
    let bytes_read = stream.read(&mut buf).await?;
    println!("Response: {}", String::from_utf8_lossy(&buf[..bytes_read]));
    Ok(())
}

async fn client_runner() {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let _ = tcp_client().await;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio::spawn(client_runner());
    let listener = TcpListener::bind("127.0.0.1:8123").await?;

    loop {
        let (mut socket, address) = listener.accept().await?;
        spawn(async move {
            println!("Connection from {address:?}");
            let mut buf = vec![0; 1024];
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
