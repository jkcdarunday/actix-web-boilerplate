use std::sync::Arc;
use std::sync::RwLock;
use app;
use actix::SyncArbiter;
use actix::Addr;
use app::db::executor::DbExecutor;

pub struct StaticAppStateData {
    pub db: Addr<DbExecutor>
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
    let db_executor = SyncArbiter::start(3, move || DbExecutor(db_pool.clone()));

    AppState {
        dynamic_data: Arc::new(RwLock::new(
            DynamicAppStateData { message: "test".to_string() }
        )),
        static_data: Arc::new(
            StaticAppStateData { db: db_executor }
        ),
    }
}
