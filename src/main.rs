mod core;
mod handlers;
mod models;
mod services;

use axum::{
    Router,
    routing::{get, post},
};
use core::app_state::AppState;
use dotenvy::dotenv;
use handlers::{
    appointments::appointments_screen_handler,
    home::home_screen_handler,
    index::index_handler,
    new_patient::{new_patient_form_handler, new_patient_screen_handler},
    new_physician::{new_physician_form_handler, new_physician_screen_handler},
    patients::{delete_patient, delete_patient_modal_handler, patients_screen_handler},
    physicians::{delete_physician, delete_physician_modal_handler, physicians_screen_handler},
};
use tower_http::services::{ServeDir, ServeFile};

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
        .route("/physicians/new", get(new_physician_screen_handler))
        .route("/new_patient", post(new_patient_form_handler))
        .route("/new_physician", post(new_physician_form_handler))
        .route(
            "/delete_patient_modal/{id}",
            get(delete_patient_modal_handler),
        )
        .route(
            "/delete_physician_modal/{id}",
            get(delete_physician_modal_handler),
        )
        .route("/delete_patient/{id}", get(delete_patient))
        .route("/delete_physician/{id}", get(delete_physician))
        .nest_service(
            "/styles",
            ServeDir::new("styles").not_found_service(ServeFile::new("/")),
        )
        .with_state(AppState::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
