use hyper::service::Service;
use hyper::{Body, Request};
use routerify::RequestServiceBuilder;

// async fn setup() {
// 	let state = libmuservice::app_state::AppState::init();
// 	// we need the test database setup
// 	// migrations run
// 	// setup seed data
// 	// start a server
// }
// #[tokio::test]
// pub async fn test_should_work() {
//     let router = libmuservice::router::build_router().await.unwrap();
//     let mut req_service = RequestServiceBuilder::new(router)
//         .unwrap()
//         .build("127.0.0.1:3000".parse().unwrap());
//     let req = Request::builder()
//         .method("GET")
//         .uri("/")
//         .body(Body::from(""))
//         .unwrap();
//     let mut resp = req_service.call(req).await.unwrap();
//     let body_bytes = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
//     let actual_body = String::from_utf8(body_bytes.to_vec()).unwrap();
//     assert_eq!(actual_body, "Hello server");
// }
