use hyper::Client;
use axum::{body::Body, http::Request};
use anyhow::Result;


// async fn setup() {
// 	let state = libmuservice::app_state::AppState::init();
// 	// we need the test database setup
// 	// migrations run
// 	// setup seed data
// 	// start a server
// }
#[tokio::test]
pub async fn test_should_work() -> Result<()> {
    let _router = libmuservice::router::build_router().await.unwrap();
    let client = Client::new();
    let req = Request::builder()
        .method("GET")
        .uri("http://127.0.0.1:3000/")
        .body(Body::from(""))
        .unwrap();
    let mut resp = client.request(req).await?;
    let body_bytes = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
    let actual_body = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert_eq!(actual_body, "Hello server\n");
    Ok(())
}
