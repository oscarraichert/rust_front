use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewPatient {
    pub name: String,
    pub address: String,
    pub phone: String,
}
