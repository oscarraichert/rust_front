use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;

use crate::{core::app_state::AppState, models::physician::Physician};

#[derive(Template, WebTemplate)]
#[template(path = "physicians.html")]
pub struct PhysiciansScreen {
    pub physicians: Vec<Physician>,
}

pub async fn physicians_screen_handler(State(state): State<AppState>) -> PhysiciansScreen {
    let physicians = state.physicians_service.get_physicians().await.unwrap();

    PhysiciansScreen {
        physicians: physicians,
    }
}
