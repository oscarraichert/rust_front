mod core;
mod handlers;
mod models;
mod services;

use axum::{Router, routing::get};
use core::app_state::AppState;
use dotenvy::dotenv;
use handlers::{
    appointments::appointments_screen_handler, home::home_screen_handler, index::index_handler,
    new_patient::new_patient_screen_handler, patients::patients_screen_handler,
    physicians::physicians_screen_handler,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/home", get(home_screen_handler))
        .route("/patients", get(patients_screen_handler))
        .route("/physicians", get(physicians_screen_handler))
        .route("/appointments", get(appointments_screen_handler))
        .route("/patients/new", get(new_patient_screen_handler))
        .with_state(AppState::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
