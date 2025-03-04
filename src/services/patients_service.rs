use crate::{core::app_state::AppState, models::patient_model::PatientModel};
use anyhow::Result;

pub async fn get_patients(state: AppState) -> Result<Vec<PatientModel>> {
    let response = state
        .http_client
        .get(state.api_host + "patients")
        .send()
        .await;

    let patients = response?.json().await?;

    Ok(patients)
}
