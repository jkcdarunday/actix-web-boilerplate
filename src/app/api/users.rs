use crate::app::db::DbPool;
use crate::app::error::AppError;
use crate::app::models::*;
use crate::schema::users;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use diesel::insert_into;
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

#[post("/users")]
pub async fn create(
    db_pool: web::Data<DbPool>,
    user: web::Json<user::User>,
) -> Result<impl Responder, AppError> {
    let mut con = db_pool.get().map_err(|e| {
        AppError::new(500)
            .cause(e)
            .message("Failed to load database")
    })?;

    let query_result = insert_into(users::table)
        .values((
            users::id.eq(user.id),
            users::username.eq(&user.username),
            users::password.eq(&user.password),
        ))
        .execute(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to create user"))?;

    Ok(HttpResponse::Ok().json(query_result))
}
