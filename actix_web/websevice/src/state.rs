use std::sync::Mutex;
use sqlx::mysql::MySqlPool;

#[derive(Debug)]
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub db: MySqlPool,
}