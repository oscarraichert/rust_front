mod components;

use axum::{
    Router,
    routing::{get, post},
};
use components::{hello_form::hello_form, index::home};
use serde::Deserialize;

#[derive(Deserialize)]
struct NameFieldValue {
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/name", post(hello_form));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
