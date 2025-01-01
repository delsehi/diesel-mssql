#![allow(warnings)]
extern crate bigdecimal;
extern crate chrono;
use crate::schema::*;
use diesel::query_dsl::LoadQuery;
use diesel::sql_types::*;
use diesel::*;

// bigint, int, smallint, tinyint, bit
#[test]
fn integer_roundtrip() {
    let connection = &mut connection();
    diesel::sql_query(
        "CREATE TABLE has_numbers (
        id INT IDENTITY(1, 1) PRIMARY KEY,
        big BIGINT,
        integer INT,
        small SMALLINT,
        tiny TINYINT,
        boo BIT
    ); 
    ",
    )
    .execute(connection)
    .unwrap();

    table! {
        has_numbers (id) {
            id -> Integer,
            big -> BigInt,
            integer -> Integer,
            small -> SmallInt,
            tiny -> SmallInt,
            boo -> Bool,
        }
    }

    #[derive(Queryable, PartialEq, Debug, Insertable, Clone)]
    struct HasNumber {
        #[diesel(skip_insertion)]
        id: i32,
        big: i64,
        integer: i32,
        small: i16,
        tiny: i16,
        boo: bool,
    }
    let input = HasNumber {
        id: 1,
        big: 9_223_372_036_854_775_807,
        integer: 2_147_483_647,
        small: 32_767,
        tiny: 255,
        boo: true,
    };

    insert_into(has_numbers::table)
        .values(input.clone())
        .execute(connection)
        .unwrap();
    let actual = has_numbers::table.load::<HasNumber>(connection).unwrap();
    assert_eq!(input, actual[0]);
}

// decimal, numeric, float, real
#[test]
fn decimal_roundtrip() {
    let connection = &mut connection();
    diesel::sql_query(
        "CREATE TABLE has_decimals (
        id INT IDENTITY(1, 1) PRIMARY KEY,
        decimal_two DECIMAL(5, 2),
        numeric_five NUMERIC(10, 5),
        float_fifty FLOAT(50),
        real_value REAL

    ); 
    ",
    )
    .execute(connection)
    .unwrap();

    table! {
        has_decimals (id) {
            id -> Integer,
            // TODO: Implement bigdecimal
            // decimal_two -> Decimal,
            // numeric_five -> Decimal,
            float_fifty -> Double,
            real_value -> Float
        }
    }

    #[derive(Queryable, PartialEq, Debug, Insertable, Clone)]
    struct HasDecimal {
        #[diesel(skip_insertion)]
        id: i32,
        // decimal_two: f64,
        // numeric_five: f64,
        float_fifty: f64,
        real_value: f32,
    }
    let input = HasDecimal {
        id: 1,
        // decimal_two: todo!(),
        float_fifty: 10.7,
        real_value: 66.42353,
    };

    insert_into(has_decimals::table)
        .values(input.clone())
        .execute(connection)
        .unwrap();
    let actual = has_decimals::table.load::<HasDecimal>(connection).unwrap();
    assert_eq!(input, actual[0]);
}

// date, time, datetime, datetime2, datetimeoffset, smalldatetime
#[test]
fn date_roundtrip() {
    let connection = &mut connection();
    diesel::sql_query(
        "CREATE TABLE has_dates (
        id INT IDENTITY(1, 1) PRIMARY KEY,
        d date,
        t time,
        dt datetime,
        dt2 datetime2,
        dto datetimeoffset,
        sdt smalldatetime

    ); 
    ",
    )
    .execute(connection)
    .unwrap();

    table! {
        has_dates (id) {
            id -> Integer,
            d -> Date,
            t -> Time,
            dt -> Timestamp,
            dt2 -> Timestamp,
            sdt -> Timestamp,
            // dt -> DateTime

        }
    }

    #[derive(Queryable, PartialEq, Debug, Insertable, Clone)]
    struct HasDate {
        #[diesel(skip_insertion)]
        id: i32,
        d: chrono::NaiveDate,
        t: chrono::NaiveTime,
        dt: chrono::NaiveDateTime,
        dt2: chrono::NaiveDateTime,
        sdt: chrono::NaiveDateTime,
    }
    let input = HasDate {
        id: 1,
        d: chrono::NaiveDate::from_ymd_opt(2024, 10, 2).unwrap(),
        t: chrono::NaiveTime::from_hms_opt(23, 30, 59).unwrap(),
        dt: chrono::NaiveDateTime::parse_from_str("2025-01-01 23:56:15", "%Y-%m-%d %H:%M:%S").unwrap(),
        dt2: chrono::NaiveDateTime::parse_from_str("2025-01-01 23:56:15", "%Y-%m-%d %H:%M:%S").unwrap(),
        sdt: chrono::NaiveDateTime::parse_from_str("2025-01-01 23:56:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    };

    insert_into(has_dates::table)
        .values(input.clone())
        .execute(connection)
        .unwrap();
    let actual = has_dates::table.load::<HasDate>(connection).unwrap();
    assert_eq!(input, actual[0]);
}

// varchar nvarchar char nchar
#[test]
fn text_roundtrip() {
    let connection = &mut connection();
    diesel::sql_query(
        "CREATE TABLE has_texts (
        id INT IDENTITY(1, 1) PRIMARY KEY,
        varc VARCHAR(MAX),
        varclimit VARCHAR(10),
        nvarc NVARCHAR(MAX),
        nvarclimit NVARCHAR(10)
        --ch CHAR 

    ); 
    ",
    )
    .execute(connection)
    .unwrap();

    table! {
        has_texts (id) {
            id -> Integer,
            varc -> Text,
            nvarc -> Text,
        }
    }

    #[derive(Queryable, PartialEq, Debug, Insertable, Clone)]
    struct HasText {
        #[diesel(skip_insertion)]
        id: i32,
        varc: String,
        nvarc: String,

    }
    let input = HasText {
        id: 1,
        varc: String::from("abcdefgh!! normal text no emojis"),
        nvarc: String::from("汉语 čevapćiči 🥰"),
    };

    insert_into(has_texts::table)
        .values(input.clone())
        .execute(connection)
        .unwrap();
    let actual = has_texts::table.load::<HasText>(connection).unwrap();
    assert_eq!(input, actual[0]);
}
