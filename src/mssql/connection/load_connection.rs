use super::bind_values_to_query;
use super::cursor::Cursor;
use super::row::MssqlRow;
use super::Mssql;
use super::MssqlBindCollector;
use super::MssqlConnection;
use crate::mssql::query_builder::MssqlQueryBuilder;
use diesel::connection::{DefaultLoadingMode, LoadConnection};
use diesel::expression::QueryMetadata;
use diesel::query_builder::{Query, QueryBuilder, QueryFragment, QueryId};
use diesel::result::QueryResult;

impl LoadConnection<DefaultLoadingMode> for MssqlConnection {
    type Row<'conn, 'query> = MssqlRow;
    type Cursor<'conn, 'query> = Cursor;
    fn load<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> QueryResult<Self::Cursor<'conn, 'query>>
    where
        T: Query + QueryFragment<Self::Backend> + QueryId + 'query,
        Self::Backend: QueryMetadata<T::SqlType>,
    {
        let mut bc = MssqlBindCollector::new();
        source.collect_binds(&mut bc, &mut (), &Mssql).unwrap();
        let mut query_builder = MssqlQueryBuilder::new();
        source.to_sql(&mut query_builder, &Mssql).unwrap();
        let sql = query_builder.finish();
        let mut query = tiberius::Query::new(sql);
        bind_values_to_query(bc.binds, &mut query);
        let query_stream = self.rt.block_on(query.query(&mut self.client));
        use diesel::result::{DatabaseErrorKind, Error};
        match query_stream {
            Err(e) => {
                return Err(Error::DatabaseError(
                    DatabaseErrorKind::Unknown,
                    Box::new(e.to_string()),
                ))
            }
            Ok(qs) => {
                let rows = self.rt.block_on(qs.into_first_result()).unwrap();
                let vecdeque = std::collections::VecDeque::from(rows);
                Ok(Cursor::new(vecdeque))
            }
        }
    }
}
