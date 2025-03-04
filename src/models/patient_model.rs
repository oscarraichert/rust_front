use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PatientModel {
    pub id: u32,
    pub name: String,
    pub address: String,
    pub phone: String,
}
