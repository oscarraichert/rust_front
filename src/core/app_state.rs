use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub api_host: String,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            http_client: Client::new(),
            api_host: std::env::var("API_HOST").unwrap(),
        }
    }
}
