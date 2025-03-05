use crate::models::physician::Physician;
use anyhow::Result;
use reqwest::Client;

#[derive(Clone)]
pub struct PhysiciansService {
    http_client: Client,
    api_host: String,
}

impl PhysiciansService {
    pub fn new(client: Client, api_host: String) -> PhysiciansService {
        PhysiciansService {
            http_client: client,
            api_host: api_host,
        }
    }

    pub async fn get_physicians(&self) -> Result<Vec<Physician>> {
        let response = self
            .http_client
            .get(format!("{}/physicians", self.api_host))
            .send()
            .await?;

        let physicians = response.json().await?;

        Ok(physicians)
    }
}
