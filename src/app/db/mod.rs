use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use once_cell::sync::Lazy;
use r2d2::{Pool, PooledConnection};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub static DB: Lazy<DbPool> = Lazy::new(|| {
    let database_url: String = crate::app::config::get("database_url");

    println!("Connecting to database {}...", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.")
});
