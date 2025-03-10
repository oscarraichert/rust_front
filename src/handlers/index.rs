use axum::response::{IntoResponse, Redirect, Response};

pub async fn index_handler() -> Response {
    let auth = true;

    if auth {
        return Redirect::to("/home").into_response();
    }

    Redirect::to("/sexo").into_response()
}
