use askama::Template;

use super::hello_form::CarecaFormTemplate;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub careca_form: CarecaFormTemplate,
}

pub async fn home() -> IndexTemplate {
    IndexTemplate {
        careca_form: CarecaFormTemplate {
            display_name: "World".to_owned(),
        },
    }
}
