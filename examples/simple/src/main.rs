use diesel::Connection;
use diesel::*;
use diesel_mssql::MssqlConnection;

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}
#[derive(Queryable, QueryableByName, Selectable, Debug)]
#[diesel(table_name = users)]
struct User {
    id: i32,
    name: String,
}

fn main() {
    let mut conn = MssqlConnection::establish("Server=tcp:localhost;Database=diesel_mssql_db;User Id=SA;Password=Aspartam123;Encrypt=true;TrustServerCertificate=true;" 
).unwrap();
    let users = users::table.load::<User>(&mut conn).unwrap();
    println!("{:#?}", users);
}
