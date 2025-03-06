use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Physician {
    pub id: u32,
    pub name: String,
    pub specialization: Specialization,
}

#[derive(Debug, Deserialize)]
pub enum Specialization {
    Cardiologist,
    Neurologist,
    Psychiatrist,
    Dermatologist,
    Gynecologist,
    Otolaryngologist,
    Pediatrist,
}

impl Display for Specialization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let specialization = match self {
            Specialization::Cardiologist => "Cardiologist",
            Specialization::Neurologist => "Neurologist",
            Specialization::Psychiatrist => "Psychiatrist",
            Specialization::Dermatologist => "Dermatologist",
            Specialization::Gynecologist => "Gynecologist",
            Specialization::Otolaryngologist => "Otolaryngologist",
            Specialization::Pediatrist => "Pediatrist",
        };

        write!(f, "{}", specialization)
    }
}
