use reqwest::Client;

use crate::services::{
    appointments_service::AppoitmentsService, patients_service::PatientsService,
    physicians_service::PhysiciansService,
};

#[derive(Clone)]
pub struct AppState {
    pub patients_service: PatientsService,
    pub physicians_service: PhysiciansService,
    pub appointments_service: AppoitmentsService,
}

impl AppState {
    pub fn new() -> Self {
        let http_client = Client::new();
        let api_host = std::env::var("API_HOST").unwrap();

        AppState {
            patients_service: PatientsService::new(http_client.clone(), api_host.clone()),
            physicians_service: PhysiciansService::new(http_client.clone(), api_host.clone()),
            appointments_service: AppoitmentsService::new(http_client.clone(), api_host.clone()),
        }
    }
}
