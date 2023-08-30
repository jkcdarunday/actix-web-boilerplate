use crate::app::api::users;
use actix_web::web;

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    cfg.service((users::list, users::create, users::delete))
}
