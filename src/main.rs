use error_stack::{ Result, ResultExt};
use libmuservice::{router, server::{self, ServerError}, app_state::AppState};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use libmuservice::settings::SETTINGS;


#[tokio::main]
async fn main() -> Result<(), ServerError> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| SETTINGS.rust_log.clone().into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = AppState::init().await.change_context(ServerError::ServerError)?;
    let router = router::build_router(app_state).await;
    server::serve(router).await.attach_printable_lazy(|| format!("Unable to serve!"))?;
    Ok(())
}
