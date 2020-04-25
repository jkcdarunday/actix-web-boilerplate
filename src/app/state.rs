use std::sync::Arc;
use std::sync::RwLock;
use crate::app;
use crate::app::db::DbPool;

pub struct StaticAppStateData {
    pub db: DbPool
}

pub struct DynamicAppStateData {
    pub message: String
}

#[derive(Clone)]
pub struct AppState {
    pub static_data: Arc<StaticAppStateData>,
    pub dynamic_data: Arc<RwLock<DynamicAppStateData>>,
}

pub fn initialize() -> AppState {
    let db_pool = app::db::get_connection_pool();

    AppState {
        dynamic_data: Arc::new(RwLock::new(
            DynamicAppStateData { message: "test".to_string() }
        )),
        static_data: Arc::new(
            StaticAppStateData { db: db_pool }
        ),
    }
}
