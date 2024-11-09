use super::super::mssql::MssqlConnection;
use crate::mssql::Mssql;
use connection::SimpleConnection;
use diesel::*;
use dotenvy::dotenv;
use serial_test::serial;

#[derive(Insertable, Queryable, QueryableByName, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Mssql))]
pub struct User {
    pub id: i32,
    pub name: String,
}

table! {
    users(id) {
        id -> Int4,
        name -> Varchar,
    }
}

pub fn setup(conn: &mut MssqlConnection) {
    conn.batch_execute("DROP TABLE IF EXISTS users;create table users(id int, name varchar(50))")
        .unwrap();
}

#[test]
#[serial]
fn can_insert_a_user() {
    dotenv().unwrap();
    let database_url = std::env::var("CONNECTION_STRING").unwrap();
    let mut conn = MssqlConnection::establish(&database_url).unwrap();
    setup(&mut conn);

    let affected_rows = diesel::insert_into(users::table)
        .values((users::columns::id.eq(1), users::columns::name.eq("Jane")))
        .execute(&mut conn)
        .unwrap();
    assert_eq!(affected_rows, 1);
}

#[test]
#[serial]
fn can_select_inserted_users() {
    dotenv().unwrap();
    let database_url = std::env::var("CONNECTION_STRING").unwrap();
    let mut conn = MssqlConnection::establish(&database_url).unwrap();
    setup(&mut conn);
    diesel::insert_into(users::table)
        .values((users::columns::id.eq(1), users::columns::name.eq("Jane")))
        .execute(&mut conn)
        .unwrap();

    let names = users::dsl::users
        .select((users::name, users::id))
        .limit(1)
        .load::<(String, i32)>(&mut conn)
        .unwrap();

    assert_eq!(names.len(), 1);
    assert_eq!(names[0].0, String::from("Jane"));
    assert_eq!(names[0].1, 1);
}
