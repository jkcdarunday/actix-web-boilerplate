use actix::Actor;
use actix::dev::Request;
use actix::SyncContext;
use actix_web::State;
use app::db::query_execute::DbQueryExecute;
use app::db::query_load::DbQueryLoad;
use app::state::AppState;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::query_builder::AsQuery;
use diesel::query_builder::Query;
use diesel::query_builder::QueryFragment;
use diesel::query_builder::QueryId;
use diesel::query_source::Queryable;
use diesel::query_source::Table;
use diesel::sql_types::HasSqlType;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::marker::PhantomData;
use futures::Future;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

// WARNING: from_err() is deprecated in futures-rs 0.3
impl DbExecutor {
    pub fn load<T: 'static, U: 'static>(state: &State<AppState>, query: T) -> Request<DbExecutor, DbQueryLoad<T, U>>
        where
            T: Send + Table,
            <T as AsQuery>::Query: QueryId + QueryFragment<Pg> + Query,
            Pg: HasSqlType<<T as AsQuery>::SqlType>,
            U: Queryable<<T as AsQuery>::SqlType, Pg> + Send {
        state.static_data.db.send(DbQueryLoad { query, _marker: PhantomData })
    }

    pub fn execute<T: 'static>(state: &State<AppState>, query: T) -> Request<DbExecutor, DbQueryExecute<T>>
        where
            T: Send + Table + QueryId + QueryFragment<Pg> + Query {
        state.static_data.db.send(DbQueryExecute { query })
    }
}
