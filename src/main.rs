extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate futures;
extern crate r2d2;
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

    println!("Listening to requests at {}...", listen_address);
    HttpServer::new(move || {
        let db_pool = crate::app::db::get_connection_pool();

        App::new()
            .app_data(db_pool)
            .configure(app::init::initialize)
            .wrap(middleware::Logger::default())
    })
        .bind(listen_address)?
        .run()
        .await
}
