use actix_web::{web};
use crate::app::route::setup_routes;

pub fn initialize(cfg: &mut web::ServiceConfig) {
    setup_routes(cfg);
}
