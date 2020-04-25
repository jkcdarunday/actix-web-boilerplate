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

use actix_web::{HttpServer, App, middleware};

pub mod app;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .configure(app::init::initialize)
            .wrap(middleware::Logger::default())
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .await
}
