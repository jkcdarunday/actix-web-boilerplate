use crate::app::db::{DbPooledConnection, DB};
use crate::app::error::AppError;

pub fn get_connection() -> Result<DbPooledConnection, AppError> {
    DB.get().map_err(|e| {
        AppError::new(500)
            .cause(e)
            .message("Failed to load database")
    })
}
