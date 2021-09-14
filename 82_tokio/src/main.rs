use tokio;
mod basic_tokio;
mod tcp_echo_server;

fn main() -> tokio::io::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
        .block_on(tcp_echo_server::run());

    Ok(())
}
