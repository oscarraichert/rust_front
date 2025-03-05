mod core;
mod handlers;
mod models;
mod services;

use axum::{Router, routing::get};
use core::app_state::AppState;
use dotenvy::dotenv;
use handlers::{
    home::home_screen, index::index, patient_list::patient_list, physician_list::physician_list,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(index))
        .route("/home", get(home_screen))
        .route("/patient_list", get(patient_list))
        .route("/physician_list", get(physician_list))
        .with_state(AppState::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
