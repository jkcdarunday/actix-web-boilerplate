use actix::Handler;
use actix::Message;
use app::db::executor::DbExecutor;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::query_builder::AsQuery;
use diesel::query_builder::Query;
use diesel::query_builder::QueryFragment;
use diesel::query_builder::QueryId;
use diesel::query_dsl::RunQueryDsl;
use diesel::query_source::Queryable;
use diesel::query_source::Table;
use diesel::result::QueryResult;
use diesel::sql_types::HasSqlType;
use std::marker::PhantomData;

pub struct DbQueryLoad<T: RunQueryDsl<PgConnection> + Table, U> {
    pub query: T,
    pub _marker: PhantomData<U>,
}

impl<T: 'static, U: 'static> Message for DbQueryLoad<T, U> where T: Table {
    type Result = QueryResult<Vec<U>>;
}

impl<T: 'static, U: 'static> Handler<DbQueryLoad<T, U>> for DbExecutor
    where
        T: Table,
        <T as AsQuery>::Query: QueryId + QueryFragment<Pg> + Query,
        Pg: HasSqlType<<T as AsQuery>::SqlType>,
        U: Queryable<<T as AsQuery>::SqlType, Pg>, {
    type Result = QueryResult<Vec<U>>;

    fn handle(&mut self, msg: DbQueryLoad<T, U>, _ctx: &mut Self::Context) -> <Self as Handler<DbQueryLoad<T, U>>>::Result {
        let connection: &PgConnection = &*(self.0.get().unwrap());
        msg.query.load::<U>(connection)
    }
}
