use anyhow::Result;
use reqwest::Client;

use crate::models::appointment::Appointment;

#[derive(Clone)]
pub struct AppoitmentsService {
    http_client: Client,
    api_host: String,
}

impl AppoitmentsService {
    pub fn new(client: Client, api_host: String) -> Self {
        AppoitmentsService {
            http_client: client,
            api_host: api_host,
        }
    }

    pub async fn get_appointments(&self) -> Result<Vec<Appointment>> {
        let response = self
            .http_client
            .get(format!("{}/appointments", self.api_host))
            .send()
            .await?;

        let appointments = response.json().await?;

        Ok(appointments)
    }
}
