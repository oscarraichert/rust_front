use serde::Deserialize;
use strum::{Display, EnumIter};

#[derive(Debug, Deserialize)]
pub struct Physician {
    pub id: u32,
    pub name: String,
    pub specialization: Specialization,
}

#[derive(Debug, Deserialize, Display, EnumIter)]
pub enum Specialization {
    Cardiologist,
    Neurologist,
    Psychiatrist,
    Dermatologist,
    Gynecologist,
    Otolaryngologist,
    Pediatrist,
}
