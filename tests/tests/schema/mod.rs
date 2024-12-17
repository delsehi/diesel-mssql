use diesel::*;
include!("mssql_schema.rs");

#[derive(
    PartialEq,
    Eq,
    Debug,
    Clone,
    Queryable,
    Identifiable,
    Insertable,
    AsChangeset,
    QueryableByName,
    Selectable,
)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}

impl User {
    pub fn new(id: i32, name: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            hair_color: None,
        }
    }

    pub fn with_hair_color(id: i32, name: &str, hair_color: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            hair_color: Some(hair_color.to_string()),
        }
    }

    pub fn new_post(&self, title: &str, body: Option<&str>) -> NewPost {
        NewPost::new(self.id, title, body)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct UserName(#[diesel(column_name = name)] pub String);

impl UserName {
    pub fn new(name: &str) -> Self {
        UserName(name.to_string())
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(Post), table_name = comments)]
pub struct Comment {
    id: i32,
    post_id: i32,
    text: String,
}

impl Comment {
    pub fn new(id: i32, post_id: i32, text: &str) -> Self {
        Comment {
            id,
            post_id,
            text: text.into(),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Queryable, Insertable, Associations, Identifiable, Selectable,
)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Post))]
#[diesel(table_name = followings)]
#[diesel(primary_key(user_id, post_id))]
pub struct Following {
    pub user_id: i32,
    pub post_id: i32,
    pub email_notifications: bool,
}

mod backend_specifics;

pub use self::backend_specifics::*;

#[derive(Debug, PartialEq, Eq, Queryable, Clone, Insertable, AsChangeset, Selectable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub hair_color: Option<String>,
}

impl NewUser {
    pub fn new(name: &str, hair_color: Option<&str>) -> Self {
        NewUser {
            name: name.to_string(),
            hair_color: hair_color.map(|s| s.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Insertable)]
#[diesel(table_name = users)]
pub struct DefaultColorUser {
    pub name: String,
    pub hair_color: Option<Option<String>>,
}

impl DefaultColorUser {
    pub fn new(name: &str, hair_color: Option<Option<&str>>) -> Self {
        Self {
            name: name.to_string(),
            hair_color: hair_color.map(|o| o.map(|s| s.to_string())),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    user_id: i32,
    title: String,
    body: Option<String>,
}

impl NewPost {
    pub fn new(user_id: i32, title: &str, body: Option<&str>) -> Self {
        NewPost {
            user_id,
            title: title.into(),
            body: body.map(|b| b.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment<'a>(
    #[diesel(column_name = post_id)] pub i32,
    #[diesel(column_name = text)] pub &'a str,
);

#[derive(PartialEq, Eq, Debug, Clone, Insertable)]
#[diesel(table_name = fk_tests)]
pub struct FkTest {
    id: i32,
    fk_id: i32,
}

impl FkTest {
    pub fn new(id: i32, fk_id: i32) -> Self {
        FkTest { id, fk_id }
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = nullable_table)]
pub struct NullableColumn {
    id: i32,
    value: Option<i32>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Queryable, Insertable, Identifiable, Associations, Selectable,
)]
#[diesel(table_name = likes)]
#[diesel(primary_key(comment_id, user_id))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Comment))]
pub struct Like {
    pub comment_id: i32,
    pub user_id: i32,
}

pub type TestConnection = diesel_mssql::MssqlConnection;

pub type TestBackend = <TestConnection as Connection>::Backend;

//Used to ensure cleanup of one-off tables, e.g. for a table created for a single test
pub struct DropTable<'a> {
    pub connection: &'a mut TestConnection,
    pub table_name: &'static str,
    pub can_drop: bool,
}

impl<'a> Drop for DropTable<'a> {
    fn drop(&mut self) {
        if self.can_drop {
            diesel::sql_query(format!("DROP TABLE {}", self.table_name))
                .execute(self.connection)
                .unwrap();
        }
    }
}

const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("migrations");

pub fn connection() -> TestConnection {
    let mut result = connection_without_transaction();
    result.begin_test_transaction().unwrap();
    result
}

pub fn connection_without_transaction() -> TestConnection {
    use diesel_migrations::MigrationHarness;
    use std::sync::Once;
    static MIGRATION_GUARD: Once = Once::new();

    let mut result = backend_specific_connection();

    MIGRATION_GUARD.call_once(|| {
        result.run_pending_migrations(MIGRATIONS).unwrap();
    });
    result
}
pub fn backend_specific_connection() -> TestConnection {
    let connection_url = dotenvy::var("MSSQL_DATABASE_URL")
        .or_else(|_| dotenvy::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set in order to run tests");
    diesel_mssql::MssqlConnection::establish(&connection_url).unwrap()
}

pub fn disable_foreign_keys(connection: &mut TestConnection) {
    diesel::sql_query("EXEC sp_MSforeachtable \"ALTER TABLE ? NOCHECK CONSTRAINT all\" ")
        .execute(connection)
        .unwrap();
}

// TODO: Add cascade for sql server, if any

pub fn drop_table_cascade(connection: &mut TestConnection, table: &str) {
    diesel::sql_query(format!("DROP TABLE {table}"))
        .execute(connection)
        .unwrap();
}

define_sql_function!(fn nextval(a: sql_types::VarChar) -> sql_types::BigInt);

pub fn connection_with_sean_and_tess_in_users_table() -> TestConnection {
    let mut connection = connection();
    insert_sean_and_tess_into_users_table(&mut connection);
    connection
}

pub fn insert_sean_and_tess_into_users_table(connection: &mut TestConnection) {
    diesel::sql_query("TRUNCATE TABLE users;INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();
    ensure_primary_key_seq_greater_than(2, connection);
}

pub fn connection_with_gilbert_and_jonathan_in_users_table() -> TestConnection {
    let mut connection = connection();
    insert_gilbert_and_jonathan_into_users_table(&mut connection);
    connection
}

pub fn insert_gilbert_and_jonathan_into_users_table(connection: &mut TestConnection) {
    diesel::sql_query("SET IDENTITY_INSERT users ON;
    INSERT INTO users (id, name, hair_color) VALUES (1, 'Gilbert', 'brown'), (2, 'Jonathan', 'electric-blue');
    SET IDENTITY_INSERT users OFF;")
        .execute(connection)
        .unwrap();
    ensure_primary_key_seq_greater_than(2, connection);
}

pub fn connection_with_nullable_table_data() -> TestConnection {
    let mut connection = connection();

    let test_data = vec![
        NullableColumn { id: 1, value: None },
        NullableColumn { id: 2, value: None },
        NullableColumn {
            id: 3,
            value: Some(1),
        },
        NullableColumn {
            id: 4,
            value: Some(2),
        },
        NullableColumn {
            id: 5,
            value: Some(1),
        },
    ];
    insert_into(nullable_table::table)
        .values(&test_data)
        .execute(&mut connection)
        .unwrap();

    connection
}

fn ensure_primary_key_seq_greater_than(_x: i64, _connection: &mut TestConnection) {
    // if cfg!(feature = "postgres") {
    //     for _ in 0..x {
    //         select(nextval("users_id_seq")).execute(connection).unwrap();
    //     }
    // }
    // TODO: Check if needed....
    // todo!()
}

pub fn find_user_by_name(name: &str, connection: &mut TestConnection) -> User {
    users::table
        .filter(users::name.eq(name))
        .first(connection)
        .unwrap()
}
