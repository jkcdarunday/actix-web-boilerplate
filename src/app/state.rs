use std::sync::Arc;
use std::sync::RwLock;
use app;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;

pub struct StaticAppStateData {
    pub db: Pool<ConnectionManager<PgConnection>>
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
    let db = app::db::get_connection_pool();

    AppState {
        dynamic_data: Arc::new(RwLock::new(
            DynamicAppStateData { message: "test".to_string() }
        )),
        static_data: Arc::new(
            StaticAppStateData { db }
        ),
    }
}
