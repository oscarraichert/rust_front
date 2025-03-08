use askama::Template;
use axum::extract::{Path, State};

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

#[derive(Template)]
#[template(path = "delete_patient_modal.html")]
pub struct DeletePatientModal {
    pub id: u32,
}

pub async fn delete_patient_modal_handler(Path(id): Path<u32>) -> DeletePatientModal {
    DeletePatientModal { id: id }
}
