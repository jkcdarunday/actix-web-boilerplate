extern crate actix;
extern crate actix_web;
extern crate futures;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate lazy_static;
extern crate config;

use actix_web::{HttpServer, App, middleware};

pub mod app;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    let listen_address: String = app::config::get("listen_address");
    let state = crate::app::state::initialize();

    println!("Listening to requests at {}...", listen_address);
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .configure(app::init::initialize)
            .wrap(middleware::Logger::default())
    })
        .bind(listen_address)?
        .run()
        .await
}
