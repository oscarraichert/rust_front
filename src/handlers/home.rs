use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeScreen;

pub async fn home_screen_handler() -> HomeScreen {
    HomeScreen
}
