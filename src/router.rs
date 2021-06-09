use hyper::{
    Body,
    Request,
    Response,
    body::to_bytes
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

async fn create_user_handler(mut req: Request<Body>) -> Result<Response<Body>> {
    let body = req.body_mut();
    let user_bytes = to_bytes(body).await?;
    let mut user = serde_json::from_slice::<User>(&user_bytes).context("Unable to parse user")?;
    let state = req.data::<AppState>().unwrap();
    match user.insert(&state.db().connection()).await {
        Ok(()) => {
            let res = Response::builder()
                .header(hyper::header::CONTENT_TYPE, "application/json")
                .status(hyper::StatusCode::CREATED)
                .body(serde_json::to_string(&user).unwrap().into())?;
            Ok(res)
        },
        Err(e) => {
            let res = Response::builder()
                .header(hyper::header::CONTENT_TYPE, "application/json")
                .status(hyper::StatusCode::UNPROCESSABLE_ENTITY)
                .body(serde_json::to_string(&e.to_string()).unwrap().into())?;
            Ok(res)
        },
    }
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
        .post("/users", create_user_handler)
        .get("/", home_handler)
        .build().unwrap();
    Ok(router)
}
