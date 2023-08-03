use crate::app::db::{DbPool, DbPooledConnection};
use crate::app::error::AppError;
use actix_web::web;

pub fn get_connection(db_pool: web::Data<DbPool>) -> Result<DbPooledConnection, AppError> {
    db_pool.get().map_err(|e| {
        AppError::new(500)
            .cause(e)
            .message("Failed to load database")
    })
}
