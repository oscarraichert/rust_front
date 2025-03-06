use askama::Template;

#[derive(Template)]
#[template(path = "new_patient.html")]
pub struct NewPatientScreen;

pub async fn new_patient_screen_handler() -> NewPatientScreen {
    NewPatientScreen
}
