use std::{sync::mpsc, time::Duration};

enum Command {
    Print(String),
}

#[tokio::main]
async fn main() {
    // Spawn a command thread for "heavy lifting"
    let (tx, rx) = mpsc::channel::<Command>();
    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<String>(10);
    let handle = tokio::runtime::Handle::current();

    std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Print(s) => {
                    let tx_reply = tx_reply.clone();
                    handle.spawn(async move {
                        tx_reply.send(s).await.unwrap();
                    });
                    // println!("{s}")
                }
            }
        }
    });

    // Receive messages
    tokio::spawn(async move {
        while let Some(reply) = rx_reply.recv().await {
            println!("{reply}");
        }
    });

    // Launch the async sender
    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send(Command::Print(format!("Hello {counter}"))).unwrap();
        counter += 1;
    }

    let (tx, mut rx) = tokio::sync::broadcast::channel::<String>(16);
    tx.send("hello".to_string());
}
