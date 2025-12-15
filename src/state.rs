use crate::config::ApiConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: ApiConfig,
}
