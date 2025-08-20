use diesel_mssql::{connection::MssqlBindCollector, query_builder::MssqlQueryBuilder};
use diesel::{
    QueryResult,
    connection::Instrumentation,
    query_builder::{QueryBuilder, QueryFragment, QueryId},
};
use diesel_async::{AnsiTransactionManager, AsyncConnection, CacheSize, SimpleAsyncConnection};
use diesel_mssql::{
    Mssql,
    connection::{MssqlRow, MssqlTransactionManager, bind_values_to_query},
};
use futures_util::future::BoxFuture;
use futures_util::stream::{self, BoxStream};
use futures_util::{FutureExt, StreamExt};
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub struct AsyncMssqlConnection {
    client: Client<Compat<TcpStream>>,
    // instrumentation: Arc<std::sync::Mutex<DynInstrumentation>>,
}

impl AsyncMssqlConnection {}

impl SimpleAsyncConnection for AsyncMssqlConnection {
    async fn batch_execute(&mut self, query: &str) -> QueryResult<()> {
        let result = self.client.simple_query(query).await;
        return match result {
            Ok(_) => Ok(()),
            Err(_error) => panic!(),
        };
    }
}

impl AsyncConnection for AsyncMssqlConnection {
    type ExecuteFuture<'conn, 'query> = BoxFuture<'conn, QueryResult<usize>>;
    type LoadFuture<'conn, 'query> = BoxFuture<'conn, QueryResult<Self::Stream<'conn, 'query>>>;
    type Stream<'conn, 'query> = BoxStream<'conn, QueryResult<Self::Row<'conn, 'query>>>;
    type Row<'conn, 'query> = MssqlRow;

    type TransactionManager = AnsiTransactionManager;
    type Backend = Mssql;

    async fn establish(database_url: &str) -> diesel::ConnectionResult<Self> {
        {
            let config = tiberius::Config::from_ado_string(database_url).unwrap();
            let tcp = tokio::net::TcpStream::connect(config.get_addr())
                .await
                .unwrap();

            tcp.set_nodelay(true).expect("Could not set no_delay");
            let client = Client::connect(config, tcp.compat_write()).await.unwrap();

            Ok(AsyncMssqlConnection { client })
        }
    }
    fn load<'conn, 'query, T>(&'conn mut self, source: T) -> Self::LoadFuture<'conn, 'query>
    where
        T: diesel::query_builder::AsQuery,
        T::Query: diesel::query_builder::QueryFragment<Self::Backend>
            + diesel::query_builder::QueryId
            + 'query,
    {
        let mut bc = MssqlBindCollector::new();
        let query = source.as_query();
        query.collect_binds(&mut bc, &mut (), &Mssql).unwrap();

        let mut query_builder = MssqlQueryBuilder::new();
        query.to_sql(&mut query_builder, &Mssql).unwrap();
        let sql = query_builder.finish();
        let mut query = tiberius::Query::new(sql);
        bind_values_to_query(bc.binds, &mut query);
        let res = async move {
            let qs = query.query(&mut self.client).await.unwrap();
            let rows = qs.into_first_result().await.unwrap();
            let rows: Vec<Result<MssqlRow, diesel::result::Error>> = rows
                .into_iter()
                .map(|r| Ok(MssqlRow { inner_row: r }))
                .collect();
            let vecdeque = std::collections::VecDeque::from(rows).into_iter();
            let s = stream::iter(vecdeque).boxed();
            Ok(s)
        };

        res.boxed()
    }

    fn execute_returning_count<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> Self::ExecuteFuture<'conn, 'query>
    where
        T: QueryFragment<Self::Backend> + QueryId + 'query,
    {
        let mut bc = MssqlBindCollector::new();
        source.collect_binds(&mut bc, &mut (), &Mssql).unwrap();
        let mut query_builder = MssqlQueryBuilder::new();
        source.to_sql(&mut query_builder, &Mssql).unwrap();
        let sql = query_builder.finish();
        let mut query = tiberius::Query::new(sql);
        bind_values_to_query(bc.binds, &mut query);
        let result = async move {
            let result = query.execute(&mut self.client).await;
            use diesel::result::{DatabaseErrorKind, Error};
            match result {
                Ok(rows_affected) => {
                    let rows_affected = rows_affected.rows_affected().first();
                    match rows_affected {
                        Some(rows_affected) => Ok(*rows_affected as usize),
                        None => Err(Error::DatabaseError(
                            DatabaseErrorKind::Unknown,
                            Box::new("Could not get rows affected".to_string()),
                        )),
                    }
                }
                Err(e) => Err(Error::DatabaseError(
                    DatabaseErrorKind::Unknown,
                    Box::new(e.to_string()),
                )),
            }
        };
        result.boxed()
    }

    fn transaction_state(&mut self) -> &mut AnsiTransactionManager {
        todo!()
    }
    fn instrumentation(&mut self) -> &mut dyn Instrumentation {
        todo!()
    }
    fn set_instrumentation(&mut self, instrumentation: impl Instrumentation) {
        todo!()
    }

    fn set_prepared_statement_cache_size(&mut self, _: CacheSize) {
        todo!()
    }
}
