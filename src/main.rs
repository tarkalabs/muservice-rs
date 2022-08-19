use anyhow::{Result, Context};
use libmuservice::{router, server};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use libmuservice::settings::SETTINGS;


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| SETTINGS.rust_log.clone().into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = router::build_router().await?;
    server::serve(router).await.context("Unable to serve")
}
