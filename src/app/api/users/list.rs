use actix_web::{get, HttpResponse, web, Responder};
use crate::app::models::*;
use crate::app::state::AppState;
use diesel::prelude::*;

#[get("/users")]
pub async fn list(state: web::Data<AppState>) -> impl Responder {
    use crate::schema::users;

    let con_result = state.static_data.db.get();
    if let Err(e) = con_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }

    let con = con_result.unwrap();
    let query_result = web::block(move || users::table.load::<user::User>(&*con).unwrap()).await;
    if let Err(e) = query_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }

    HttpResponse::Ok().json(query_result.unwrap())
}

