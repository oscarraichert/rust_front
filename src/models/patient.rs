use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Patient {
    pub id: u32,
    pub name: String,
    pub address: String,
    pub phone: String,
}
