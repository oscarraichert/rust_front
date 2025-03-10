use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "home.html")]
pub struct HomeScreen;

pub async fn home_screen_handler() -> HomeScreen {
    HomeScreen
}
