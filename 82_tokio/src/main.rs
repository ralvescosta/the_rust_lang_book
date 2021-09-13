use tokio;
mod basic_tokio;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    basic_tokio::run().await;
    Ok(())
}
