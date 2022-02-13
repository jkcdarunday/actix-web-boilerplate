use actix_web::{get, HttpResponse, web, Error};
use crate::app::models::*;
use crate::app::state::AppState;
use diesel::prelude::*;

#[get("/users")]
pub async fn list(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    use crate::schema::users;

    let con = state.static_data.db.get()
        .expect("Failed to retrieve DB connection from pool");

    let users = web::block(move || users::table.load::<user::User>(&*con))
        .await
        .map_err(|e| {
            HttpResponse::InternalServerError().body(format!("{}", e))
        })?;

    Ok(HttpResponse::Ok().json(users))
}

