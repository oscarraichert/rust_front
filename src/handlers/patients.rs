use askama::Template;
use axum::extract::State;

use crate::{core::app_state::AppState, models::patient::Patient};

#[derive(Template)]
#[template(path = "patients.html")]
pub struct PatientsScreen {
    pub patients: Vec<Patient>,
}

pub async fn patients_screen_handler(State(state): State<AppState>) -> PatientsScreen {
    let patients = state.patients_service.get_patients().await.unwrap();

    PatientsScreen { patients: patients }
}
