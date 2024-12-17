use super::schema::*;
use diesel::*;

#[test]
fn simple_distinct() {
    use crate::schema::users::dsl::*;

    let connection = &mut connection();
    diesel::sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Sean'), ('Tess')")
        .execute(connection)
        .unwrap();

    let source = users.select(name).distinct().order(name);
    let expected_data = vec!["Sean".to_string(), "Tess".to_string()];
    let data: Vec<String> = source.load(connection).unwrap();

    assert_eq!(expected_data, data);
}
