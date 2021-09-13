use tokio;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let result = tokio::join!(
        tokio::spawn(async {
            println!("1 - Started");
            println!("1 - Do some jobs! - await 3s...");
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            println!("1 - Finish my job");
        }),
        tokio::spawn(async {
            println!("2 - Started");
            println!("2 - Do some jobs! - await 3s...");
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            println!("2 - Finish my job");
        }),
        tokio::spawn(async {
            println!("3 - Started");
            println!("3 - Do some jobs! - await 3s...");
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            println!("3 - Finish my job");
        })
    );

    Ok(())
}
