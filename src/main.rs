use color_eyre::{eyre::WrapErr, Result};
use libmuservice::{router, server, app_state::AppState};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use libmuservice::settings::SETTINGS;
use tracing::instrument;

#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| SETTINGS.rust_log.clone().into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(ErrorLayer::default())
        .init();
    color_eyre::install()?;

    let app_state = AppState::init().await?;
    let router = router::build_router(app_state).await?;
    server::serve(router).await.context("Unable to serve")
}
