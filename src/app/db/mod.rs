use r2d2::Pool;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

pub mod executor;
pub mod query_execute;
pub mod query_load;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new("postgres://localhost/actix_test");

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
