use askama::Template;
use axum::Form;

use crate::models::new_patient::NewPatient;

#[derive(Template)]
#[template(path = "new_patient.html")]
pub struct NewPatientScreen;

pub async fn new_patient_screen_handler() -> NewPatientScreen {
    NewPatientScreen
}

pub async fn new_patient_form_handler(Form(form): Form<NewPatient>) {
    let patient = NewPatient {
        name: form.name,
        address: form.address,
        phone: form.phone,
    };

    println!("{:?}", patient)
}
