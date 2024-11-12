use std::vec;

use super::super::mssql::MssqlConnection;
use connection::SimpleConnection;
use diesel::*;
use dotenvy::dotenv;
use serial_test::serial;

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
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
    }
}

allow_tables_to_appear_in_same_query!(users, comments, posts);
joinable!(comments -> posts (post_id));
joinable!(posts -> users (user_id));

fn setup(conn: &mut MssqlConnection) {
    conn.batch_execute(
        r"
    DROP TABLE IF EXISTS users;
    DROP TABLE IF EXISTS posts;
    DROP TABLE IF EXISTS comments;
    CREATE TABLE users(
    id INT identity(1,1) NOT NULL, 
    -- Strange bug here. NVARCHAR does not work
    name VARCHAR(100) NOT NULL, 
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
    text NVARCHAR(MAX) NOT NULL
    );
    ",
    )
    .unwrap();
    for name in vec!["Delfi", "Georg", "Jane"] {
        insert_into(users::table)
            .values(users::name.eq(name))
            .execute(conn)
            .unwrap();
    }
    for post in vec![
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
    let users: Vec<String> = users::table
        .inner_join(posts::table)
        .select(users::name)
        .load::<String>(&mut conn)
        .unwrap();

    assert_eq!(users[0], "Georg");
    assert_eq!(users[1], "Delfi");
}
