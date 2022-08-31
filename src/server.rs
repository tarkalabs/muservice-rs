use color_eyre::{Report, Result, eyre::Context};
use axum::{Router, Server};
use hyper::{Body};
use std::net::SocketAddr;
use std::str::FromStr;
use tracing::log::info;

use crate::settings::SETTINGS;

pub async fn serve(router: Router<Body>) -> Result<()>{
    let addr = SocketAddr::from_str(&format!("{}:{}", SETTINGS.host, SETTINGS.port))?;
    let builder = Server::try_bind(&addr)?;
    info!("Server started listening on {}", addr);
    match builder
        .serve(router.into_make_service())
        .await {
            Err(e) => Err(Report::new(e)),
            Ok(rs) => Ok(rs)
    }.context("Unable to create router service")?;
    Ok(())
}
