use anyhow::{Result, Context};
use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;
use libmuservice::{router, server};


#[tokio::main]
async fn main() -> Result<()> {
    let subscribe = FmtSubscriber::new();
    set_global_default(subscribe).context("Unable to setup fmt subscriber")?;

    let router = router::build_router().await?;
    server::serve(router).await.context("Unable to serve")
}
