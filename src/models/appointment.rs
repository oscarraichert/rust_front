use super::{patient::Patient, physician::Physician};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Appointment {
    pub id: u32,
    pub patient: Patient,
    pub physician: Physician,
    pub date: String,
}
