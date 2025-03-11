use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};

use crate::{
    core::app_state::AppState,
    models::{patient::AppError, physician::Physician},
};

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

#[derive(Template, WebTemplate)]
#[template(path = "delete_physician_modal.html")]
pub struct DeletePhysicianModal {
    pub physician: Physician,
}

pub async fn delete_physician_modal_handler(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> DeletePhysicianModal {
    let physician = state.physicians_service.get_physician(id).await.unwrap();

    DeletePhysicianModal {
        physician: physician,
    }
}

pub async fn delete_physician(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<PhysiciansScreen, AppError> {
    state.physicians_service.delete_physician(id).await.unwrap();

    let physicians = state.physicians_service.get_physicians().await.unwrap();

    Ok(PhysiciansScreen {
        physicians: physicians,
    })
}
