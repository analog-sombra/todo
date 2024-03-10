use axum::{response::Html, response::Json, routing::get, serve, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[tokio::main]
async fn main() {
    server().await;

    // println!("Hello, world!");
}

async fn server() {
    let routes_hello: Router =
        Router::new().route("/hello", get(|| async { Html("Hello, world!") }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    serve(listener, routes_hello).await.unwrap();
}
