use anyhow::{Result, Context};
use axum::{
    http::Request,
    body::Body,
    routing::{get, post},
    Extension,
    Router, response::{Response, IntoResponse},
    
};
use crate::db::User;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use axum::{
    Json,
    http::StatusCode
};

use crate::app_state::{self, AppState};

async fn home_handler() -> Json<String> {
    Json(String::from("Hello server"))
}

async fn users_handler(req: Request<Body>) -> Result<Json<Vec<User>>, StatusCode> {
    let state = req.extensions().get::<AppState>().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let users = User::all(&state.db().connection())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(users))
}

async fn create_user_handler(Json(mut payload): Json<User>, Extension(state): Extension<AppState>) -> Result<Response, StatusCode> {
    payload.insert(&state.db().connection())
        .await
        .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((
        // set status code
        StatusCode::CREATED,
        [("Content-Type", "application/json")]
    ).into_response())
}

// async fn logger(req: Request<Body>) -> Result<Request<Body>> {
//     info!("{} {} {}", req.remote_addr(), req.method(), req.uri().path());
//     Ok(req)
// }

pub async fn build_router() -> Result<Router<Body>> {
    let shared_state = app_state::AppState::init().await.context("error initializing state")?;
    let router = Router::new()
    .layer(
        ServiceBuilder::new()
            .layer(Extension(shared_state))
            .layer(TraceLayer::new_for_http())
    )
    .route("/", get(home_handler))
    .route("/users", get(users_handler))
    .route("/users", post(create_user_handler));
    Ok(router)
}
