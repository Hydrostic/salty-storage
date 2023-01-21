use super::config::AppConfig;
use rbatis::Rbatis;
#[derive(Clone)]
pub struct AppState {
    pub db: Rbatis,
    pub settings: AppConfig,
}
impl AppState {
    pub fn new(db: Rbatis, settings: AppConfig) -> Self {
        AppState { db, settings }
    }
}
