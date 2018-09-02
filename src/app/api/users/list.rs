use actix_web::State;
use actix_web::HttpResponse;
use app::models::user::User;
use app::state::AppState;
use diesel::prelude::*;
use futures::Future;
use actix_web::error::ErrorInternalServerError;
use actix_web::{FutureResponse, AsyncResponder};
use app::db::executor::DbExecutor;

pub fn list(state: (State<AppState>)) -> FutureResponse<HttpResponse> {
    use schema::users;

    DbExecutor::load(&state, users::table)
        .from_err()
        .and_then(|result: QueryResult<Vec<User>>| {
            let items = result
                .map_err(|_| ErrorInternalServerError("Error loading users"))
                .unwrap();

            Ok(HttpResponse::Ok().json(items))
        })
        .responder()
}
