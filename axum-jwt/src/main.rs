use axum::{routing::get, Router};
use tokio::net::TcpListener;
use serde::{Serialize, Deserialize};
mod routes;
mod services;
mod auth;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    let app = Router::new()
        .route("/hello", get(|| async { "Hello world!" }));

    println!("Listening on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}