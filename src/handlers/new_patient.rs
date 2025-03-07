use anyhow::Result;
use askama::Template;
use axum::{Form, extract::State};

use crate::{
    core::app_state::AppState,
    models::{
        new_patient::NewPatient,
        patient::{AppError, Patient},
    },
};

#[derive(Template)]
#[template(path = "new_patient.html")]
pub struct NewPatientScreen;

pub async fn new_patient_screen_handler() -> NewPatientScreen {
    NewPatientScreen
}

#[axum::debug_handler]
pub async fn new_patient_form_handler(
    State(state): State<AppState>,
    Form(form): Form<NewPatient>,
) -> Result<Patient, AppError> {
    let new_patient = NewPatient {
        name: form.name,
        address: form.address,
        phone: form.phone,
    };

    let patient = state.patients_service.create_patient(new_patient).await?;

    Ok(patient)
}
