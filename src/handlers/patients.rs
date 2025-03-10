use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};

use crate::{
    core::app_state::AppState,
    models::patient::{AppError, Patient},
};

#[derive(Template, WebTemplate)]
#[template(path = "patients.html")]
pub struct PatientsScreen {
    pub patients: Vec<Patient>,
}

pub async fn patients_screen_handler(State(state): State<AppState>) -> PatientsScreen {
    let patients = state.patients_service.get_patients().await.unwrap();

    PatientsScreen { patients: patients }
}

#[derive(Template, WebTemplate)]
#[template(path = "delete_patient_modal.html")]
pub struct DeletePatientModal {
    pub patient: Patient,
}

pub async fn delete_patient_modal_handler(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> DeletePatientModal {
    let patient = state.patients_service.get_patient(id).await.unwrap();

    DeletePatientModal { patient: patient }
}

pub async fn delete_patient(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<PatientsScreen, AppError> {
    state.patients_service.delete_patient(id).await.unwrap();

    let patients = state.patients_service.get_patients().await.unwrap();

    Ok(PatientsScreen { patients: patients })
}
