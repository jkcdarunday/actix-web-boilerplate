use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new("postgres://localhost/actix_test");

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
