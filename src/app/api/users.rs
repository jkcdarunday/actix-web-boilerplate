use crate::app::db::DbPool;
use crate::app::models::*;
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;

#[get("/users")]
pub async fn list(db_pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::users;

    let con_result = db_pool.get();
    if let Err(e) = con_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }

    let mut con = con_result.unwrap();
    let query_result =
        web::block(move || users::table.load::<user::User>(&mut *con).unwrap()).await;
    if let Err(e) = query_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }

    HttpResponse::Ok().json(query_result.unwrap())
}
