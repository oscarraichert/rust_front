use askama::Template;
use axum::extract::State;

use crate::{core::app_state::AppState, models::physician::Physician};

#[derive(Template)]
#[template(path = "physician_list.html")]
pub struct PhysicianListScreen {
    pub physicians: Vec<Physician>,
}

pub async fn physician_list(State(state): State<AppState>) -> PhysicianListScreen {
    let physicians = state.physicians_service.get_physicians().await.unwrap();

    PhysicianListScreen {
        physicians: physicians,
    }
}
