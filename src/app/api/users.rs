use crate::app::db::DbPool;
use crate::app::error::AppError;
use crate::app::models::*;
use crate::schema::users;
use actix_web::{get, web, HttpResponse, Responder, Result};
use diesel::prelude::*;

#[get("/users")]
pub async fn list(db_pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut con = db_pool.get().map_err(|e| {
        AppError::new(500)
            .cause(e)
            .message("Failed to load database")
    })?;

    let query_result = users::table
        .load::<user::User>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to load users"))?;

    Ok(HttpResponse::Ok().json(query_result))
}
