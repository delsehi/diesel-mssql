mod bind_collector;
mod cursor;
mod load_connection;
mod row;
mod transaction_manager;

use crate::mssql::query_builder::MssqlQueryBuilder;
pub use bind_collector::{BindValue, MssqlBindCollector};
use diesel::{
    connection::{
        ConnectionSealed, Instrumentation, InstrumentationEvent, SimpleConnection,
        TransactionManager,
    },
    migration::MigrationConnection,
    query_builder::{QueryBuilder, QueryFragment, QueryId},
    Connection, QueryResult, RunQueryDsl,
};
use tiberius::{Client, Query};
use tokio::{net::TcpStream, runtime::Runtime};
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};
use transaction_manager::MssqlTransactionManager;

use super::Mssql;

pub struct MssqlConnection {
    client: Client<Compat<TcpStream>>,
    transaction_state: MssqlTransactionManager,
    instrumentation: Option<Box<dyn Instrumentation>>,
    rt: Runtime,
}

impl SimpleConnection for MssqlConnection {
    fn batch_execute(&mut self, query: &str) -> diesel::QueryResult<()> {
        if let Some(i) = &mut self.instrumentation {
            i.on_connection_event(InstrumentationEvent::start_query(
                &diesel::connection::StrQueryHelper::new(query),
            ));
        }
        let _ = self
            .rt
            .block_on(self.client.simple_query(query))
            // TODO: Handle this error
            .unwrap_or_else(|_| panic!("Query failed: {}", query));
        Ok(())
    }
}

impl ConnectionSealed for MssqlConnection {}

impl Connection for MssqlConnection {
    fn transaction<T, E, F>(&mut self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
        E: From<diesel::result::Error>,
    {
        Self::TransactionManager::transaction(self, f)
    }

    fn begin_test_transaction(&mut self) -> QueryResult<()> {
        match Self::TransactionManager::transaction_manager_status_mut(self) {
            diesel::connection::TransactionManagerStatus::Valid(valid_status) => {
                std::assert_eq!(None, valid_status.transaction_depth())
            }
            diesel::connection::TransactionManagerStatus::InError => {
                std::panic!("Transaction manager in error")
            }
        };
        Self::TransactionManager::begin_transaction(self)?;
        // set the test transaction flag
        // to prevent that this connection gets dropped in connection pools
        // Tests commonly set the poolsize to 1 and use `begin_test_transaction`
        // to prevent modifications to the schema
        Self::TransactionManager::transaction_manager_status_mut(self).set_test_transaction_flag();
        Ok(())
    }

    fn test_transaction<T, E, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
        E: std::fmt::Debug,
    {
        let mut user_result = None;
        let _ = self.transaction::<(), _, _>(|conn| {
            user_result = f(conn).ok();
            Err(diesel::result::Error::RollbackTransaction)
        });
        user_result.expect("Transaction did not succeed")
    }

    type Backend = Mssql;

    type TransactionManager = MssqlTransactionManager;

    fn establish(database_url: &str) -> diesel::ConnectionResult<Self> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            // TODO: Handle error
            .expect("Could not create tokio runtime");
        let config = tiberius::Config::from_ado_string(database_url).unwrap();
        let tcp = rt
            .block_on(tokio::net::TcpStream::connect(config.get_addr()))
            // TODO: Handle error
            .expect("Could not create tcp stream");
        tcp.set_nodelay(true).expect("Could not set no_delay");
        let client = rt
            .block_on(Client::connect(config, tcp.compat_write()))
            .expect("Could not connect to client");
        let transaction_state = MssqlTransactionManager {
            ..Default::default()
        };

        let instrumentation = diesel::connection::get_default_instrumentation();
        Ok(MssqlConnection {
            client,
            rt,
            instrumentation,
            transaction_state,
        })
    }

    fn execute_returning_count<T>(&mut self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Self::Backend> + QueryId,
    {
        let mut bc = MssqlBindCollector::new();
        source.collect_binds(&mut bc, &mut (), &Mssql).unwrap();
        let mut query_builder = MssqlQueryBuilder::new();
        source.to_sql(&mut query_builder, &Mssql).unwrap();
        let sql = query_builder.finish();
        let my_sql = sql.clone();
        let mut query = Query::new(sql);
        for b in bc.binds.into_iter() {
            // b.bind_to_query(&mut query);
            match b {
                BindValue::Integer(val) => {
                    query.bind(*val);
                }
                BindValue::Text(val) => {
                    query.bind(val);
                }
                BindValue::Date(val) => {
                    query.bind(*val);
                }

                BindValue::Bool(val) => query.bind(*val),
                BindValue::NotSet(_) => todo!(),
                BindValue::Bigint(val) => {
                    query.bind(*val);
                }
                BindValue::Binary(val) => {
                    query.bind(val);
                }
                // BindValue::Double() => {
                //     query.bind(*val);
                // },
                BindValue::Decimal(val) => {
                    query.bind(*val);
                }
                BindValue::Float(val) => {
                    query.bind(*val);
                }
                BindValue::SmallInt(val) => {
                    query.bind(*val);
                }
                BindValue::Time(val) => {
                    query.bind(*val);
                }
                BindValue::Timestamp(val) => {
                    query.bind(*val);
                }
            }
        }
        // bc.binds.into_iter().for_each(|b| {
        //     b.bind_to_query(&mut query);
        // });
        let result = self
            .rt
            .block_on(query.execute(&mut self.client))
            .expect(&my_sql);
        let rows_affected = *result.rows_affected().first().unwrap() as usize;
        Ok(rows_affected)
    }

    fn transaction_state(
        &mut self,
    ) -> &mut <Self::TransactionManager as TransactionManager<Self>>::TransactionStateData {
        &mut self.transaction_state
    }

    fn instrumentation(&mut self) -> &mut dyn Instrumentation {
        &mut self.instrumentation
    }

    fn set_instrumentation(&mut self, instrumentation: impl diesel::connection::Instrumentation) {
        self.instrumentation = Some(Box::new(instrumentation));
    }
}

pub const CREATE_MIGRATIONS_TABLE: &str = include_str!("setup_migration_table.sql");
impl MigrationConnection for MssqlConnection {
    fn setup(&mut self) -> QueryResult<usize> {
        diesel::sql_query(CREATE_MIGRATIONS_TABLE).execute(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::RunQueryDsl;
    // extern crate dotenvy;
    use dotenvy;

    #[test]
    fn can_establish_connection() -> Result<(), diesel::ConnectionError> {
        dotenvy::dotenv().expect("Can get ");
        let conn_str = std::env::var("CONNECTION_STRING").unwrap();
        MssqlConnection::establish(&conn_str)?;
        Ok(())
    }

    #[test]
    fn can_execute() {
        dotenvy::dotenv().expect("");
        let conn_str = std::env::var("CONNECTION_STRING").unwrap();
        let mut c = MssqlConnection::establish(&conn_str).unwrap();
        c.batch_execute("DROP TABLE IF EXISTS delfi").ok();
        c.batch_execute("CREATE TABLE delfi (id INT, name VARCHAR(50))")
            .ok();
        let affected_rows =
            diesel::sql_query("insert into delfi (id, name) values (1, 'delfi'), (2, 'georg')")
                .execute(&mut c)
                .unwrap();
        c.batch_execute("DROP TABLE IF EXISTS delfi").ok();
        assert_eq!(2, affected_rows);
    }

    #[test]
    fn simple_connection() {
        dotenvy::dotenv().expect("");
        let conn_str = std::env::var("CONNECTION_STRING").unwrap();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Whopsie");
        // let inner = rt.block_on(future)
        let config = tiberius::Config::from_ado_string(&conn_str).unwrap();
        let tcp = rt
            .block_on(tokio::net::TcpStream::connect(config.get_addr()))
            .expect("msg");
        tcp.set_nodelay(true).expect("noo");
        let client = rt
            .block_on(Client::connect(config, tcp.compat_write()))
            .unwrap();
        let transaction_state = MssqlTransactionManager {
            ..Default::default()
        };
        let mut conn = MssqlConnection {
            client,
            rt,
            instrumentation: None,
            transaction_state,
        };
        conn.batch_execute("SELECT 1").unwrap();
    }
}
