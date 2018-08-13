extern crate actix;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use actix_web::server;

pub mod app;
pub mod schema;

//#[derive(Serialize, Deserialize, Debug)]
//struct TestQuery {
//    pub message: String
//}


fn main() {
    let state = app::state::initialize();

    server::new(move || app::init::initialize(state.clone()))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
