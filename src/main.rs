use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, address) = listener.accept().await.unwrap();

        let (tx, mut rx) = (tx.clone(), tx.subscribe());

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), address)).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (message, other_address) = result.unwrap();
                        if other_address != address {
                            writer.write_all(message.as_bytes()).await.unwrap();
                        }
                    }
                };
            }
        });
    }
}
