use anyhow::{Result, Context};
// use tracing::subscriber::set_global_default;
// use tracing_subscriber::FmtSubscriber;
use libmuservice::{router, server};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() -> Result<()> {
    //Old code for subscriber
    // let subscribe = FmtSubscriber::new();
    // set_global_default(subscribe).context("Unable to setup fmt subscriber")?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "muservice=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    print!("RUST_LOG: {:?}\n", std::env::var("RUST_LOG").context("No RUST_LOG"));

    let router = router::build_router().await?;
    server::serve(router).await.context("Unable to serve")
}
