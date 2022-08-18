use anyhow::{Result, Context, Error};
use axum::{Router, Server};
use hyper::{Body};
use std::net::SocketAddr;
use std::str::FromStr;
use tracing::log::info;

use crate::settings::SETTINGS;

pub async fn serve(router: Router<Body>) -> Result<()>{
    let addr = SocketAddr::from_str(&format!("{}:{}", SETTINGS.host, SETTINGS.port))?;
    info!("Server started listening on {}", addr);
    match Server::bind(&addr)
        .serve(router.into_make_service())
        .await {
            Err(e) => Err(Error::msg(e.to_string())),
            Ok(rs) => Ok(rs)
    }.context("Unable to create router service")?;
    Ok(())
}
