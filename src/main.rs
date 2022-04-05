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

use actix_web::{HttpServer, App, web::Data, middleware::Logger};

pub mod app;
pub mod schema;

#[actix_web::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    let listen_address: String = app::config::get("listen_address");
    let state = crate::app::state::initialize();

    println!("Listening to requests at {}...", listen_address);
    HttpServer::new(move || {
        let state_data = Data::new(state.clone());
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(state_data)
            .configure(app::init::initialize)
    })
        .bind(listen_address)?
        .run()
        .await
}
