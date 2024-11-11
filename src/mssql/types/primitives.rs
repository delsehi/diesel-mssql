use super::Mssql;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types;
use std::io::prelude::*;
use tiberius::ColumnData;

// bool
impl FromSql<sql_types::Bool, Mssql> for bool {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::Bit(Some(val)) = bytes {
            return Ok(val);
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

// i16
impl FromSql<sql_types::SmallInt, Mssql> for i16 {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::I16(Some(val)) = bytes {
            return Ok(val);
        };
        unimplemented!()
    }
}
impl ToSql<sql_types::SmallInt, Mssql> for i16 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.write_all(&[*self as u8])
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}
// i32
impl FromSql<sql_types::Integer, Mssql> for i32 {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::I32(Some(val)) = bytes {
            return Ok(val);
        };
        unimplemented!()
    }
}
impl ToSql<sql_types::Integer, Mssql> for i32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.write_all(&[*self as u8])
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}
// i64
impl FromSql<sql_types::Bigint, Mssql> for i64 {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::I64(Some(val)) = bytes {
            return Ok(val);
        };
        // TODO: Why does count return i32 instead of i64?
        if let ColumnData::I32(Some(val)) = bytes {
            return Ok(val as i64);
        };
        unimplemented!()
    }
}
impl ToSql<sql_types::Bigint, Mssql> for i64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.write_all(&[*self as u8])
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}
// f32
impl FromSql<sql_types::Float, Mssql> for f32 {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::F32(Some(val)) = bytes {
            return Ok(val);
        };
        unimplemented!()
    }
}
impl ToSql<sql_types::Float, Mssql> for f32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.write_all(&[*self as u8])
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}
// f64
impl FromSql<sql_types::Decimal, Mssql> for f64 {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::F64(Some(val)) = bytes {
            return Ok(val);
        };
        unimplemented!()
    }
}
impl ToSql<sql_types::Decimal, Mssql> for f64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.write_all(&[*self as u8])
            .map(|_| IsNull::No)
            .map_err(Into::into)
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

