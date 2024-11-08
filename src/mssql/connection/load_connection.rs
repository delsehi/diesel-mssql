use std::borrow::BorrowMut;

use crate::mssql::query_builder::MssqlQueryBuilder;

use super::cursor::Cursor;
use super::row::MssqlRow;
use super::MssqlConnection;
use super::{cursor, Mssql};
use diesel::connection::{DefaultLoadingMode, LoadConnection};
use diesel::expression::QueryMetadata;
use diesel::query_builder::bind_collector::RawBytesBindCollector;
use diesel::query_builder::{Query, QueryBuilder, QueryFragment, QueryId};
use diesel::query_dsl::methods::LoadQuery;
use diesel::result::QueryResult;
use diesel::row::RowIndex;
use diesel::row::{Field, Row, RowSealed};
use tiberius::{Column, ColumnData};

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
        let mut bc = RawBytesBindCollector::<Mssql>::new();
        source.collect_binds(&mut bc, &mut (), &Mssql).unwrap();
        let mut query_builder = MssqlQueryBuilder::new();
        source.to_sql(&mut query_builder, &Mssql).unwrap();
        let sql = query_builder.finish();
        let mut query = tiberius::Query::new(sql);
        bc.binds.into_iter().for_each(|b| {
            query.bind(b);
        });
        let query_stream = self.rt.block_on(query.query(&mut self.client)).unwrap();
        let rows = self.rt.block_on(query_stream.into_first_result()).unwrap();
        Ok(Cursor::new(rows))
    }
}
