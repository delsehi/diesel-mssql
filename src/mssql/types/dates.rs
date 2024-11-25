use super::super::connection::BindValue;
use super::Mssql;
use chrono::prelude::*;
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, Result, ToSql};
use diesel::sql_types::{Date, Time, Timestamp};

impl ToSql<Date, Mssql> for NaiveDate {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> Result {
        out.set_value(BindValue::Date(self));
        Ok(IsNull::No)
    }
}
impl FromSql<Date, Mssql> for NaiveDate {
    fn from_sql(
        bytes: <Mssql as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        if let tiberius::ColumnData::DateTime2(d) = bytes {
            if let Some(i) = d {
                let d = i.date();
                let dt = chrono::NaiveDate::from_num_days_from_ce_opt(d.days() as i32);
                return Ok(dt.unwrap());
            }
        };
        diesel::deserialize::Result::Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Noo",
        )))
    }
}

impl ToSql<Timestamp, Mssql> for NaiveDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> Result {
        out.set_value(BindValue::Timestamp(self));
        Ok(IsNull::No)
    }
}

impl ToSql<Time, Mssql> for NaiveTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mssql>) -> Result {
        out.set_value(BindValue::Time(self));
        Ok(IsNull::No)
    }
}
