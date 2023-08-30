use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use actix_web_validator::Path;
use diesel::prelude::*;
use validator::Validate;

use crate::app::error::AppError;
use crate::app::lib::helpers::get_connection;
use crate::app::lib::validators::user_id_exists;
use crate::app::models::*;
use crate::schema::users;

#[get("/users")]
pub async fn list() -> Result<impl Responder, AppError> {
    let mut con = get_connection()?;

    let query_result = users::table
        .load::<user::User>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to load users"))?;

    Ok(HttpResponse::Ok().json(query_result))
}

#[post("/users")]
pub async fn create(user: web::Json<user::User>) -> Result<impl Responder, AppError> {
    let mut con = get_connection()?;

    let query_result = diesel::insert_into(users::table)
        .values((
            users::id.eq(user.id),
            users::username.eq(&user.username),
            users::password.eq(&user.password),
        ))
        .execute(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to create user"))?;

    Ok(HttpResponse::Ok().json(query_result))
}

#[derive(Deserialize, Validate)]
pub struct DeleteRequest {
    #[validate(custom = "user_id_exists")]
    id: i32,
}

#[delete("/users/{id}")]
pub async fn delete(path: Path<DeleteRequest>) -> Result<impl Responder, AppError> {
    let mut con = get_connection()?;

    let query_result = diesel::delete(users::table.filter(users::id.eq(path.id)))
        .execute(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to delete user"))?;

    Ok(HttpResponse::Ok().json(query_result))
}
