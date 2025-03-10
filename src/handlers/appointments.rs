use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;

use crate::{core::app_state::AppState, models::appointment::Appointment};

#[derive(Template, WebTemplate)]
#[template(path = "appointments.html")]
pub struct AppointmentsScreen {
    appointments: Vec<Appointment>,
}

pub async fn appointments_screen_handler(State(state): State<AppState>) -> AppointmentsScreen {
    let appointments = state.appointments_service.get_appointments().await.unwrap();

    AppointmentsScreen {
        appointments: appointments,
    }
}
