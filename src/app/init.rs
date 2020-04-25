use actix_web::{web};
use crate::app::route::setup_routes;

pub fn initialize(cfg: &mut web::ServiceConfig) {
    let state = crate::app::state::initialize();
    setup_routes(cfg.data(state.clone()));
}
