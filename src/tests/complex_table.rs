use super::super::mssql::MssqlConnection;
use diesel::connection::SimpleConnection;
use diesel::*;
use dotenvy::dotenv;
use serial_test::serial;

table! {
    people (id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
        birthday -> Nullable<Date>
    }
}
table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        body -> Nullable<Text>,
    }
}

table! {
    comments (id) {
        id -> Integer,
        post_id -> Integer,
        text -> Text,
        created -> Timestamp
    }
}

#[derive(Queryable, Debug)]
#[diesel(table_name = people)]
struct People {
    id: i32,
    name: String,
    hair_color: Option<String>,
    birthday: Option<chrono::NaiveDate>,
}

allow_tables_to_appear_in_same_query!(people, comments, posts);
joinable!(comments -> posts (post_id));
joinable!(posts -> people (user_id));

fn setup(conn: &mut MssqlConnection) {
    conn.batch_execute(
        r"
    DROP TABLE IF EXISTS people;
    DROP TABLE IF EXISTS posts;
    DROP TABLE IF EXISTS comments;
    CREATE TABLE people(
    id INT identity(1,1) NOT NULL, 
    -- Strange bug here. NVARCHAR does not work
    name VARCHAR(100) NOT NULL, 
    birthday DATETIME2 NULL,
    hair_color VARCHAR(100) NULL);
    CREATE TABLE posts(
    id INT identity(1,1) NOT NULL,
    user_id INT NOT NULL,
    title NVARCHAR(256) NOT NULL,
    body NVARCHAR(MAX) NULL
    );
    CREATE TABLE comments(
    id INT identity(1,1) NOT NULL,
    post_id INT NOT NULL,
    created DATETIME2 NOT NULL,
    text NVARCHAR(MAX) NOT NULL
    );
    ",
    )
    .unwrap();
    for name in ["Delfi", "Georg", "Jane"] {
        insert_into(people::table)
            .values((
                people::name.eq(name),
                people::birthday.eq(chrono::NaiveDate::from_ymd_opt(1995, 2, 17).unwrap()),
            ))
            .execute(conn)
            .unwrap();
    }
    for post in [
        (1, "Is this Diesel?", "I am writing this post"),
        (
            1,
            "What is an ORM",
            "It means snake in swedish what is it in english",
        ),
        (
            2,
            "Should Diesel conquer the world",
            "We have the power to make this real. What do you think?",
        ),
    ] {
        insert_into(posts::table)
            .values((
                posts::user_id.eq(post.0),
                posts::title.eq(post.1),
                posts::body.eq(post.2),
            ))
            .execute(conn)
            .unwrap();
    }
}

#[test]
#[serial]
fn can_join_users_and_posts() {
    dotenv().unwrap();
    let database_url = std::env::var("CONNECTION_STRING").unwrap();
    let mut conn = MssqlConnection::establish(&database_url).unwrap();
    setup(&mut conn);
    let users: Vec<String> = people::table
        .inner_join(posts::table)
        .select(people::name)
        .load::<String>(&mut conn)
        .unwrap();

    assert_eq!(users[0], "Delfi");
    assert_eq!(users[1], "Georg");
}

#[test]
#[serial]
fn can_get_all() {
    dotenv().unwrap();
    let database_url = std::env::var("CONNECTION_STRING").unwrap();
    let mut conn = MssqlConnection::establish(&database_url).unwrap();
    setup(&mut conn);
    let users = people::table.load::<People>(&mut conn).unwrap();
    assert_eq!(users[0].hair_color, None);
    // assert_eq!(users[0].name, "Delfi");
    // assert_eq!(users[1].name, "Georg");
    let birthday = users[0].birthday.unwrap();
    assert_eq!(birthday.to_string(), String::from("1995-02-17"));
}
