use hyper::{
    Body,
    Request,
    Response,
};
use routerify::{Router, Middleware};
use routerify::prelude::*;
use anyhow::{Error, Result, Context};
use crate::db::User;
use tracing::info;

use crate::app_state::{self, AppState};

async fn home_handler(_: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello server")))
}

async fn users_handler(req: Request<Body>) -> Result<Response<Body>> {
    let state = req.data::<AppState>().unwrap();
    let users = User::all(&state.db().connection()).await?;
    let body = serde_json::to_string(&users)?;
    let resp = Response::builder()
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(Body::from(body))?;
    Ok(resp)
}

async fn logger(req: Request<Body>) -> Result<Request<Body>> {
    info!("{} {} {}", req.remote_addr(), req.method(), req.uri().path());
    Ok(req)
}

pub async fn build_router() -> Result<Router<Body, Error>> {
    let state = app_state::AppState::init().await.context("error initializing state")?;
    let router = Router::builder()
        .data(state)
        .middleware(Middleware::pre(logger))
        .get("/users", users_handler)
        .get("/", home_handler)
        .build().unwrap();
    Ok(router)
}
