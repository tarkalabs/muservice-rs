use hyper::{Server, Request, Body, Response};
use std::net::SocketAddr;
use routerify::{Router, RouterService};
use anyhow::{Result, Context, Error};


pub async fn serve(router: Router<Body, Error>) -> Result<()>{
    let rs = match RouterService::new(router) {
      Err(e) => Err(Error::msg(e.to_string())),
      Ok(rs) => Ok(rs)
    }.context("unable to create router service")?;
    let addr = "0.0.0.0:3000".parse().context("unable to parse socket address")?;
    if let Err(e) = Server::bind(&addr).serve(rs).await {
        eprintln!("Server error {}", e);
    }
    Ok(())
  }