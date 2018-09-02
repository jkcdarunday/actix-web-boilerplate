#![feature(specialization)]
extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate futures;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use actix_web::server;

pub mod app;
pub mod schema;


fn main() {
    let sys = actix::System::new("server");

    let state = app::state::initialize();

    server::new(move || app::init::initialize(state.clone()))
        .bind("127.0.0.1:8088")
        .unwrap()
        .start();

    let _ = sys.run();
}
