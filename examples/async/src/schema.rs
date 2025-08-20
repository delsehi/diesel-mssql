use diesel::table;

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}
