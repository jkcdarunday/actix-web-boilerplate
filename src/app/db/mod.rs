use r2d2::Pool;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new("postgres://localhost/actix_test");

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
