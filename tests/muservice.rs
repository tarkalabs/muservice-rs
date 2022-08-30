use axum::{body::Body, http::Request};
use http::StatusCode;
use libmuservice::{app_state::AppState, db::DB, model::User};
use sqlx::PgPool;
use tower::{ServiceExt, Service};
use std::net::{SocketAddr, TcpListener};

#[tokio::test]
async fn test_should_work() {
    let listener = TcpListener::bind("127.0.0.1:5000".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();
    let app_state = AppState::init().await.unwrap();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(libmuservice::router::build_router(app_state).await.unwrap().into_make_service())
            .await
            .unwrap();
    });

    let client = hyper::Client::new();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{}/", addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let actual_body = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert_eq!(actual_body, "Hello server\n");
}

#[sqlx::test]
async fn test_create_user_handler(pool: PgPool) {
    let db = DB::new_with_pool(pool);
    let app_state = AppState::init_with_db(db);
    let mut router = libmuservice::router::build_router(app_state).await.unwrap();

    let user = User { id: None, name: "userman".to_string(), email: "email@email.com".to_string() };

    let request = Request::builder()
        .method("POST")
        .uri("/users")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&user).unwrap()))
        .unwrap();
    let response = router.ready().await.unwrap().call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[sqlx::test]
async fn test_users_handler_empty(pool: PgPool) {
    let db = DB::new_with_pool(pool);
    let app_state = AppState::init_with_db(db);
    let mut router = libmuservice::router::build_router(app_state).await.unwrap();

    let request = Request::builder()
        .uri("/users")
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();
    let response = router.ready().await.unwrap().call(request).await.unwrap();
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let users: Vec<User> = serde_json::from_slice(&body).unwrap();
    assert_eq!(users.len(), 0);
}

#[sqlx::test(migrations = "./test_migrations")]
async fn test_users_handler_has_user(pool: PgPool) {
    let db = DB::new_with_pool(pool);
    let app_state = AppState::init_with_db(db);
    let mut router = libmuservice::router::build_router(app_state).await.unwrap();

    let request = Request::builder()
        .uri("/users")
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();
    let response = router.ready().await.unwrap().call(request).await.unwrap();
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let users: Vec<User> = serde_json::from_slice(&body).unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].email, "email@email.com");
}
