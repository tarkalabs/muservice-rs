use std::str::FromStr;
use hyper::{Server, Body};
use std::net::SocketAddr;
use routerify::{Router, RouterService};
use anyhow::{Result, Context, Error};
use crate::settings::SETTINGS;

pub async fn serve(router: Router<Body, Error>) -> Result<()>{
    let rs = match RouterService::new(router) {
        Err(e) => Err(Error::msg(e.to_string())),
        Ok(rs) => Ok(rs)
    }.context("unable to create router service")?;
    let addr = SocketAddr::from_str(&format!("{}:{}", SETTINGS.host, SETTINGS.port))
        .context("Unable to parse host / port")?;
    if let Err(e) = Server::bind(&addr).serve(rs).await {
        eprintln!("Server error {}", e);
    }
    Ok(())
}
