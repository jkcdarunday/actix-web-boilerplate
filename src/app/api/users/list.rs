use actix_web::{get, HttpResponse, web::block, web::Data, Error};
use crate::app::models::*;
use crate::app::state::AppState;
use diesel::prelude::*;

#[get("/users")]
pub async fn list(state: Data<AppState>) -> Result<HttpResponse, Error> {
    let users_list = block(move || {
        use crate::schema::users::dsl::*;

        let conn = state.static_data.db.get()
            .expect("Failed to retrieve DB connection from pool");

        users.load::<user::User>(&*conn)
    })
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match users_list {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => {
            let res = HttpResponse::InternalServerError().body(format!("{}", e));
            Ok(res)
        }
    }
}

