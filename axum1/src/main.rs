use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("Listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello world!"
}