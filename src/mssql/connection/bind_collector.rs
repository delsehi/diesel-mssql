use crate::mssql::Mssql;
use diesel::{
    QueryResult,
    query_builder::BindCollector,
    sql_types::{HasSqlType, TypeMetadata},
};

pub struct MssqlBindCollector<'a> {
    pub(crate) binds: Vec<BindValue<'a>>,
}

pub enum BindValue<'a> {
    Bigint(&'a i64),
    Binary(&'a [u8]),
    Bool(&'a bool),
    Decimal(&'a f64),
    Double(&'a f64),
    Float(&'a f32),
    Integer(&'a i32),
    SmallInt(&'a i16),
    TinyInt(&'a u8),
    Text(&'a str),
    #[cfg(feature = "chrono")]
    Time(&'a chrono::NaiveTime),
    #[cfg(feature = "chrono")]
    Timestamp(&'a chrono::NaiveDateTime),
    #[cfg(feature = "chrono")]
    Date(&'a chrono::NaiveDate),
    NotSet(tiberius::ColumnType),
}

impl Default for MssqlBindCollector<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl MssqlBindCollector<'_> {
    pub fn new() -> Self {
        Self { binds: vec![] }
    }
}

impl<'a> BindCollector<'a, Mssql> for MssqlBindCollector<'a> {
    type Buffer = BindValue<'a>;

    fn push_bound_value<T, U>(
        &mut self,
        bind: &'a U,
        metadata_lookup: &mut <Mssql as TypeMetadata>::MetadataLookup,
    ) -> QueryResult<()>
    where
        Mssql: HasSqlType<T>,
        U: diesel::serialize::ToSql<T, Mssql> + ?Sized + 'a,
    {
        let metadata = Mssql::metadata(metadata_lookup);
        let out = BindValue::NotSet(metadata);
        let mut out = diesel::serialize::Output::<Mssql>::new(out, metadata_lookup);
        bind.to_sql(&mut out).unwrap();
        let res = out.into_inner();

        self.binds.push(res);

        Ok(())
    }
}
