use app::models::user::User;
use diesel::pg::PgConnection;
use actix_web::Responder;
use actix_web::Json;
use actix_web::State;
use app::state::AppState;
use diesel::prelude::*;

pub fn list(state: (State<AppState>)) -> impl Responder {
    use schema::users;

    let connection = state.static_data.db.get().unwrap();
    let result = users::table
        .load::<User>(&*connection)
        .expect("Failed to get users");

    Json(result.to_vec())
}
