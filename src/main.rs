mod components;
mod core;
mod models;
mod screens;
mod services;

use axum::{Router, routing::get};
use core::app_state::AppState;
use dotenvy::dotenv;
use screens::index::index;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(index))
        .with_state(AppState::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
