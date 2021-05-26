use hyper::{Request, Body, Response};
use std::net::SocketAddr;
use routerify::{Router, RouterService};
use std::convert::Infallible;

mod db;

type Result<T> = std::result::Result<T, Infallible>;

async fn home_handler(_: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello server")))
}
#[tokio::main]
async fn main() -> Result<()> {
    let router = Router::builder()
    .get("/", home_handler)
    .build().unwrap();
    let rs =RouterService::new(router).unwrap();
    let addr = "0.0.0.0:3000".parse().unwrap();
    if let Err(e) = hyper::Server::bind(&addr).serve(rs).await {
        eprintln!("Server error {}", e);
    }
    Ok(())
}
