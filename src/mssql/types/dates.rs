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
        match bytes {
            tiberius::ColumnData::Date(Some(date)) => {
                let dt = NaiveDate::from_ymd_opt(1, 1, 1).unwrap()
                    + chrono::Duration::days(date.days().into());
                return Ok(dt);
            }
            _ => {}
        }
        // TODO: Better error handling
        diesel::deserialize::Result::Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not deserialize date",
        )))
    }
}

// Taken from Tiberius. TODO: Move this functionality to just use it in Tiberius.
#[inline]
fn from_days(days: i64, start_year: i32) -> NaiveDate {
    NaiveDate::from_ymd_opt(start_year, 1, 1).unwrap() + chrono::Duration::days(days)
}

#[inline]
fn from_sec_fragments(sec_fragments: i64) -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        + chrono::Duration::nanoseconds(sec_fragments * (1e9 as i64) / 300)
}
#[inline]
fn from_mins(mins: u32) -> NaiveTime {
    NaiveTime::from_num_seconds_from_midnight_opt(mins, 0).unwrap()
}

impl FromSql<Timestamp, Mssql> for NaiveDateTime {
    fn from_sql(
        bytes: <Mssql as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes {
            tiberius::ColumnData::DateTime(Some(dt)) => {
                return Ok(NaiveDateTime::new(
                    from_days(dt.days() as i64, 1900),
                    from_sec_fragments(dt.seconds_fragments() as i64),
                ));
            }
            tiberius::ColumnData::SmallDateTime(Some(dt)) => {
                return Ok(NaiveDateTime::new(
                    from_days(dt.days() as i64, 1900),
                    from_mins(dt.seconds_fragments() as u32 * 60),
                ))
            }
            tiberius::ColumnData::DateTime2(Some(dt)) => {
                return Ok(NaiveDateTime::new(
                    from_days(dt.date().days() as i64, 1),
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                        + chrono::Duration::nanoseconds(
                            dt.time().increments() as i64 * 10i64.pow(9 - dt.time().scale() as u32),
                        ),
                ))
            }
            tiberius::ColumnData::DateTimeOffset(dt) => todo!(),
            _ => {}
        }

        diesel::deserialize::Result::Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not deserialize date",
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

impl diesel::deserialize::FromSql<Time, Mssql> for NaiveTime {
    fn from_sql(
        bytes: <Mssql as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes {
            tiberius::ColumnData::Time(Some(time)) => {
                let ns = time.increments() as i64 * 10i64.pow(9 - time.scale() as u32);
                let test =
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap() + chrono::Duration::nanoseconds(ns);
                return Ok(test);
            }
            _ => {}
        }
        // TODO: Better error handling
        diesel::deserialize::Result::Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not deserialize date",
        )))
    }
}
