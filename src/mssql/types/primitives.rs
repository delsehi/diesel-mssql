use super::super::connection::BindValue;
use super::Mssql;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types;
use tiberius::ColumnData;

const SERIALIZE_ERROR_MSG: &str = "Could not serialize value";

// bool
impl FromSql<sql_types::Bool, Mssql> for bool {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::Bit(Some(val)) = bytes {
            return Ok(val);
        };
        if let ColumnData::Bit(None) = bytes {}
        diesel::deserialize::Result::Err(SERIALIZE_ERROR_MSG.into())
    }
}

impl ToSql<sql_types::Bool, Mssql> for bool {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.set_value(BindValue::Bool(self));
        Ok(IsNull::No)
    }
}

// i16
impl FromSql<sql_types::SmallInt, Mssql> for i16 {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::I16(Some(val)) = bytes {
            return Ok(val);
        };
        diesel::deserialize::Result::Err(SERIALIZE_ERROR_MSG.into())
    }
}
impl ToSql<sql_types::SmallInt, Mssql> for i16 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.set_value(BindValue::SmallInt(self));
        Ok(IsNull::No)
    }
}
// i32
impl FromSql<sql_types::Integer, Mssql> for i32 {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::I32(Some(val)) = bytes {
            return Ok(val);
        };
        diesel::deserialize::Result::Err(SERIALIZE_ERROR_MSG.into())
    }
}

impl ToSql<sql_types::Integer, Mssql> for i32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.set_value(BindValue::Integer(self));
        Ok(IsNull::No)
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
        diesel::deserialize::Result::Err(SERIALIZE_ERROR_MSG.into())
    }
}
impl ToSql<sql_types::Bigint, Mssql> for i64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.set_value(BindValue::Bigint(self));
        Ok(IsNull::No)
    }
}
// f32
impl FromSql<sql_types::Float, Mssql> for f32 {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::F32(Some(val)) = bytes {
            return Ok(val);
        };
        diesel::deserialize::Result::Err(SERIALIZE_ERROR_MSG.into())
    }
}
impl ToSql<sql_types::Float, Mssql> for f32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.set_value(BindValue::Float(self));
        Ok(IsNull::No)
    }
}
// f64
impl FromSql<sql_types::Decimal, Mssql> for f64 {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::F64(Some(val)) = bytes {
            return Ok(val);
        };
        diesel::deserialize::Result::Err(SERIALIZE_ERROR_MSG.into())
    }
}
impl ToSql<sql_types::Decimal, Mssql> for f64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.set_value(BindValue::Decimal(self));
        Ok(IsNull::No)
    }
}
// Text
impl FromSql<sql_types::Text, Mssql> for String {
    fn from_sql(bytes: ColumnData<'_>) -> deserialize::Result<Self> {
        if let ColumnData::String(Some(val)) = bytes {
            return Ok(val.to_string());
        };
        if let ColumnData::String(None) = bytes {
            // TODO: Why can this be empty?
            // Strange error. Nullable strings will still try to become a value.
            return Ok("".to_string());
        }
        diesel::deserialize::Result::Err(SERIALIZE_ERROR_MSG.into())
    }
}

impl ToSql<sql_types::Text, Mssql> for str {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> diesel::serialize::Result {
        out.set_value(BindValue::Text(self));
        Ok(IsNull::No)
    }
}
