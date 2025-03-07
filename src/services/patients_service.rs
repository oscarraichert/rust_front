use crate::models::{new_patient::NewPatient, patient::Patient};
use anyhow::{Ok, Result};
use reqwest::Client;

#[derive(Clone)]
pub struct PatientsService {
    http_client: Client,
    api_host: String,
}

impl PatientsService {
    pub fn new(client: Client, api_host: String) -> PatientsService {
        PatientsService {
            http_client: client,
            api_host: api_host,
        }
    }

    pub async fn get_patients(&self) -> Result<Vec<Patient>> {
        let response = self
            .http_client
            .get(format!("{}/patients", self.api_host))
            .send()
            .await?;

        let patients = response.json().await?;

        Ok(patients)
    }

    pub async fn create_patient(&self, new_patient: NewPatient) -> Result<Patient> {
        let response = self
            .http_client
            .post(format!("{}/patients", self.api_host))
            .json(&new_patient)
            .send()
            .await?;

        let patient = response.json().await?;

        Ok(patient)
    }
}
