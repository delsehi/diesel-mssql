use diesel::sql_types::Text;
use diesel::{insert_into, prelude::*, sql_query};
use diesel_async::{AsyncConnection, RunQueryDsl};
use diesel_mssql::AsyncMssqlConnection;
use schema::users;
mod schema;

const DATABASE_URL: &str = "Server=tcp:localhost;Database=diesel_mssql_db;User Id=SA;Password=Aspartam123;Encrypt=true;TrustServerCertificate=true;";

#[derive(Queryable, QueryableByName, Selectable, Insertable, Debug)]
#[diesel(table_name = users)]
struct User {
    // id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let mut conn = AsyncMssqlConnection::establish(DATABASE_URL).await.unwrap();
    // let users = users::table.load::<User>(&mut conn).await.unwrap();
    let new_users = vec![User {
        // id: 1,
        name: "Cool".to_string(),
    }];
    insert_into(users::table).values(new_users).execute(&mut conn).await.unwrap();
    // let users = users::table.load::<User>(&mut conn).await.unwrap();
    println!("Hello, world!");
    let users = sql_query("SELECT * FROM users WHERE name = @P1")
        .bind::<Text, _>("Delfi")
        .load::<User>(&mut conn)
        .await
        .unwrap();
    for user in users.iter() {
        println!("{:#?}", user);
        println!("{0}", user.name);
    }
    println!("Hello, world!");
    let data: Vec<User> = users::table
        .filter(users::id.gt(0))
        .or_filter(users::name.like("Delfi"))
        .select(User::as_select())
        .load(&mut conn)
        .await
        .unwrap();

    println!("{:#?}", data);
}
