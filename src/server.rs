use axum::{Router, Server};
use error_stack::{Report, IntoReport, ResultExt, Result};
use hyper::{Body};
use thiserror::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use tracing::log::info;

use crate::settings::SETTINGS;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Server returned an error!")]
    ServerError
}

pub async fn serve(router: Router<Body>) -> Result<(), ServerError>{
    let addr = SocketAddr::from_str(&format!("{}:{}", SETTINGS.host, SETTINGS.port))
        .report()
        .change_context(ServerError::ServerError)?;
    info!("Server started listening on {}", addr);
    let builder = Server::try_bind(&addr).report().change_context(ServerError::ServerError)?;
        match builder.serve(router.into_make_service())
        .await {
            Err(e) => Err(Report::new(e).change_context(ServerError::ServerError)), //Maybe hyper::Error?
            Ok(rs) => Ok(rs)
    }.attach_printable_lazy(|| "Unable to create router service!")?;
    Ok(())
}
