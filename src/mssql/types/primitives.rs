use super::Mssql;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types;
use std::io::prelude::*;
use tiberius::ColumnData;

impl FromSql<sql_types::Bool, Mssql> for bool {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::Bit(Some(val)) = bytes {
            return Ok(val);
        };
        unimplemented!()
    }
}

impl FromSql<sql_types::Integer, Mssql> for i32 {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::I32(Some(val)) = bytes {
            return Ok(val);
        };
        unimplemented!()
    }
}
impl FromSql<sql_types::Text, Mssql> for String {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::String(Some(val)) = bytes {
            return Ok(val.to_string());
        };
        unimplemented!()
    }
}

impl ToSql<sql_types::Bool, Mssql> for bool {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.write_all(&[*self as u8])
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}

impl ToSql<sql_types::BigInt, Mssql> for i64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.write_all(&[*self as u8])
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}

impl ToSql<sql_types::Integer, Mssql> for i32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.write_all(&[*self as u8])
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}

// impl ToSql<BigInt, Mssql> for i64 {
//     fn to_sql<'b>(
//         &'b self,
//         out: &mut diesel::serialize::Output<'b, '_, Mssql>,
//     ) -> serialize::Result {
//         out.set_value(self);
//         Ok(serialize::IsNull::No)
//     }
// }

// impl FromSql<Bigint, Mssql> for i64 {
//     fn from_sql(
//         bytes: <Mssql as diesel::backend::Backend>::RawValue<'_>,
//     ) -> deserialize::Result<Self> {
//         if let crate::mssql::connection::tiberius_value::InnerValue::BigInt(a) = bytes.inner {
//             return Ok(a);
//         };
//         Err("Got an invalid value for i64".into())
//     }
// }
