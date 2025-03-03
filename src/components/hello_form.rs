use askama_axum::Template;
use axum::Form;

use crate::NameFieldValue;

#[derive(Template)]
#[template(path = "hello_form.html")]
pub struct CarecaFormTemplate {
    pub display_name: String,
}

pub async fn hello_form(Form(payload): Form<NameFieldValue>) -> CarecaFormTemplate {
    CarecaFormTemplate {
        display_name: payload.name,
    }
}
