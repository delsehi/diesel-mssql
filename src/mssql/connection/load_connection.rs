use super::cursor::Cursor;
use super::row::MssqlRow;
use super::BindValue;
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
        let debug_sql = sql.clone();
        let mut query = tiberius::Query::new(sql);
        // TODO: Make this a function
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
        let query_stream = self
            .rt
            .block_on(query.query(&mut self.client))
            .expect(&debug_sql);
        let rows = self.rt.block_on(query_stream.into_first_result()).unwrap();
        let vecdeque = std::collections::VecDeque::from(rows);
        Ok(Cursor::new(vecdeque))
    }
}
