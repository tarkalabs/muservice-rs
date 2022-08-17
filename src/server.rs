use std::str::FromStr;
use axum::{Router, Server};
use hyper::{Body};
use std::net::SocketAddr;
use anyhow::{Result, Context, Error};
use crate::settings::SETTINGS;
use tracing::info;

pub async fn serve(router: Router<Body>) -> Result<()>{
    let addr = SocketAddr::from_str(&format!("{}:{}", SETTINGS.host, SETTINGS.port))?;
    info!("Server started listening on {}", addr);
    match Server::bind(&addr)
        .serve(router.into_make_service())
        .await {
            Err(e) => Err(Error::msg(e.to_string())),
            Ok(rs) => Ok(rs)
    }.context("unable to create router service")?;
    Ok(())
}
