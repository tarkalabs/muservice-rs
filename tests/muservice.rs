use axum::{body::Body, http::Request};
use std::net::{SocketAddr, TcpListener};

#[tokio::test]
    async fn test_should_work() {
        let listener = TcpListener::bind("127.0.0.1:5000".parse::<SocketAddr>().unwrap()).unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(libmuservice::router::build_router().await.unwrap().into_make_service())
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
