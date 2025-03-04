use askama::Template;
use axum::extract::State;

use crate::{
    core::app_state::AppState, models::patient_model::PatientModel, services::patients_service,
};

#[derive(Template)]
#[template(path = "patient_list.html")]
pub struct PatientListTemplate {
    pub patients: Vec<PatientModel>,
}

pub async fn patient_list(State(state): State<AppState>) -> PatientListTemplate {
    let patients = patients_service::get_patients(state).await.unwrap();

    PatientListTemplate { patients: patients }
}
