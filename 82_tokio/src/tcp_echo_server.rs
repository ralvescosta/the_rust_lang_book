use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

pub async fn run() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let (reader, mut writer) = socket.split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            let bytes_read = reader.read_line(&mut line).await.unwrap();
            writer.write_all(line.as_bytes()).await.unwrap();
            line.clear();
        }
    }
}
