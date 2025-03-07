use askama_axum::IntoResponse;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Patient {
    pub id: u32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

impl IntoResponse for Patient {
    fn into_response(self) -> askama_axum::Response {
        let body = self;

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> askama_axum::Response {
        let body = self;

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError(value)
    }
}
