use crate::mssql::Mssql;
use diesel::{query_builder::BindCollector, sql_types::HasSqlType};

pub struct MssqlBindCollector<'a> {
    pub(crate) binds: Vec<BindValue<'a>>,
}

pub enum BindValue<'a> {
    Bigint(&'a i64),
    Binary(&'a [u8]),
    Bool(&'a bool),
    // Double(),
    Decimal(&'a f64),
    Float(&'a f32),
    Integer(&'a i32),
    SmallInt(&'a i16),
    Text(&'a str),
    #[cfg(feature = "chrono")]
    Time(&'a chrono::NaiveTime),
    #[cfg(feature = "chrono")]
    Timestamp(&'a chrono::NaiveDateTime),
    #[cfg(feature = "chrono")]
    Date(&'a chrono::NaiveDate),
    NotSet(tiberius::ColumnType),
}

impl<'a> BindValue<'a> {
    pub fn bind_to_query(self, query: &'a mut tiberius::Query<'a>) {
        match self {
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
}

impl<'a> Default for MssqlBindCollector<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> MssqlBindCollector<'a> {
    pub fn new() -> Self {
        Self { binds: vec![] }
    }
}

impl<'a> BindCollector<'a, Mssql> for MssqlBindCollector<'a> {
    type Buffer = BindValue<'a>;

    fn push_bound_value<T, U>(
        &mut self,
        bind: &'a U,
        metadata_lookup: &mut <Mssql as diesel::sql_types::TypeMetadata>::MetadataLookup,
    ) -> diesel::QueryResult<()>
    where
        Mssql: diesel::backend::Backend + diesel::sql_types::HasSqlType<T>,
        U: diesel::serialize::ToSql<T, Mssql> + ?Sized + 'a,
    {
        let e = Mssql::metadata(metadata_lookup);
        let out = BindValue::NotSet(e);
        let mut out = diesel::serialize::Output::<Mssql>::new(out, metadata_lookup);
        bind.to_sql(&mut out).unwrap();
        let res = out.into_inner();

        self.binds.push(res);

        Ok(())
    }
}
