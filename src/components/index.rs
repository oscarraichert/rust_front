use askama::Template;
use axum::extract::State;

use crate::core::app_state::AppState;

use super::patient_list::{PatientListTemplate, patient_list};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<T: Template> {
    pub body: T,
}

pub async fn index(state: State<AppState>) -> IndexTemplate<PatientListTemplate> {
    IndexTemplate {
        body: patient_list(state).await,
    }
}
