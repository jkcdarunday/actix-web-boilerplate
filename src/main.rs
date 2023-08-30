extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate r2d2;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate lazy_static;
extern crate config;
extern crate dotenv;

use crate::app::db::DB;
use actix_web::{middleware, App, HttpServer};

pub mod app;
pub mod schema;

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    let listen_address: String = app::config::get("listen_address");

    DB.get().expect("Failed to connect to database.");

    println!("Listening to requests at {}...", listen_address);
    HttpServer::new(move || {
        App::new()
            .configure(app::init::initialize)
            .wrap(middleware::Logger::default())
    })
    .bind(listen_address)?
    .run()
    .await
}
