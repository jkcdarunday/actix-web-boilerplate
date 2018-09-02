use actix::Handler;
use actix::Message;
use app::db::executor::DbExecutor;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::query_builder::Query;
use diesel::query_builder::QueryFragment;
use diesel::query_builder::QueryId;
use diesel::query_dsl::RunQueryDsl;
use diesel::query_source::Table;
use diesel::result::QueryResult;

pub struct DbQueryExecute<T: RunQueryDsl<PgConnection> + Table> {
    pub query: T
}

impl<T: 'static> Message for DbQueryExecute<T> where T: Table {
    type Result = QueryResult<usize>;
}

impl<T: 'static> Handler<DbQueryExecute<T>> for DbExecutor where T: Table + QueryId + QueryFragment<Pg> + Query {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: DbQueryExecute<T>, _ctx: &mut Self::Context) -> <Self as Handler<DbQueryExecute<T>>>::Result {
        let connection: &PgConnection = &*(self.0.get().unwrap());
        msg.query.execute(connection)
    }
}
