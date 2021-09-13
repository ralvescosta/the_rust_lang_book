use tokio;
mod basic_tokio;
mod tcp_echo_server;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    tcp_echo_server::run().await;
    Ok(())
}
