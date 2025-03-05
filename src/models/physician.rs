use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Physician {
    pub id: u32,
    pub name: String,
    pub specialization: String,
}
