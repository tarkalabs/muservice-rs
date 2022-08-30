use std::error::Error;

use error_stack::{IntoReport, Result, ResultExt};
use libmuservice::{router, server, app_state::AppState};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use libmuservice::settings::SETTINGS;


#[tokio::main]
async fn main() -> Result<(), impl Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| SETTINGS.rust_log.clone().into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = AppState::init().await?;
    let router = router::build_router(app_state).await?;
    server::serve(router).await.report().attach_printable_lazy(|| format!("Unable to serve!"))?
}
