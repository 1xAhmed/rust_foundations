#[tracing::instrument]
async fn hello_world() {
    println!("Hello!");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Starting up");
    tracing::warn!("Are you sure this is a good idea?");
    tracing::error!("Something went horribly wrong");
    hello_world().await;

    Ok(())
}