use askama::Template;
use askama_web::WebTemplate;
use axum::{Form, extract::State};
use strum::IntoEnumIterator;

use crate::{
    core::app_state::AppState,
    models::{new_physician::NewPhysician, physician::Specialization},
};

use super::physicians::PhysiciansScreen;

#[derive(Template, WebTemplate)]
#[template(path = "new_physician.html")]
pub struct NewPhysicianScreen {
    specializations: Vec<Specialization>,
}

pub async fn new_physician_screen_handler() -> NewPhysicianScreen {
    NewPhysicianScreen {
        specializations: Specialization::iter().collect(),
    }
}

pub async fn new_physician_form_handler(
    State(state): State<AppState>,
    Form(form): Form<NewPhysician>,
) -> PhysiciansScreen {
    let new_physician = NewPhysician {
        name: form.name,
        specialization: form.specialization,
    };

    let _ = state
        .physicians_service
        .create_physician(new_physician)
        .await;

    let physicians = state.physicians_service.get_physicians().await.unwrap();

    PhysiciansScreen { physicians }
}
