use askama::Template;
use axum::extract::State;

use crate::{core::app_state::AppState, models::patient::Patient};

#[derive(Template)]
#[template(path = "patient_list.html")]
pub struct PatientListScreen {
    pub patients: Vec<Patient>,
}

pub async fn patient_list(State(state): State<AppState>) -> PatientListScreen {
    let patients = state.patients_service.get_patients().await.unwrap();

    PatientListScreen { patients: patients }
}
