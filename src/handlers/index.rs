use askama_axum::{IntoResponse, Response};
use axum::response::Redirect;

pub async fn index() -> Response {
    let auth = true;

    if auth {
        return Redirect::to("/home").into_response();
    }

    Redirect::to("/sexo").into_response()
}
