use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewPatient {
    pub name: String,
    pub address: String,
    pub phone: String,
}
