use crate::schema::*;
use diesel::result::Error;
use diesel::*;

#[test]
#[ignore = "The statements are blocking"] // Is this possible in SQL Server?
                                          // Manually doing this with two connections will keep each other blocking..
fn transaction_executes_fn_in_a_sql_transaction() {
    const TEST_NAME: &str = "transaction_executes_fn_in_a_sql_transaction";
    let conn1 = &mut connection_without_transaction();
    let conn2 = &mut connection_without_transaction();
    setup_test_table(conn1, TEST_NAME);

    fn get_count(conn: &mut TestConnection) -> i64 {
        count_test_table(conn, TEST_NAME)
    }

    conn1
        .transaction::<_, Error, _>(|conn1| {
            assert_eq!(0, get_count(conn1));
            assert_eq!(0, get_count(conn2));
            diesel::sql_query(format!("INSERT INTO {TEST_NAME} DEFAULT VALUES")).execute(conn1)?;
            assert_eq!(1, get_count(conn1));
            assert_eq!(0, get_count(conn2));
            Ok(())
        })
        .unwrap();

    assert_eq!(1, get_count(conn1));
    assert_eq!(1, get_count(conn2));

    drop_test_table(conn1, TEST_NAME);
}

#[test]
fn transaction_returns_the_returned_value() {
    let conn1 = &mut connection_without_transaction();

    assert_eq!(Ok(1), conn1.transaction::<_, Error, _>(|_| Ok(1)));
}

#[test]
fn transaction_is_rolled_back_when_returned_an_error() {
    let connection = &mut connection_without_transaction();
    let test_name = "transaction_is_rolled_back_when_returned_an_error";
    setup_test_table(connection, test_name);

    let _ = connection.transaction::<(), _, _>(|connection| {
        diesel::sql_query(format!("INSERT INTO {test_name} DEFAULT VALUES"))
            .execute(connection)
            .unwrap();
        Err(Error::RollbackTransaction)
    });
    assert_eq!(0, count_test_table(connection, test_name));

    drop_test_table(connection, test_name);
}

#[test]
fn transactions_can_be_nested() {
    let connection = &mut connection_without_transaction();
    const TEST_NAME: &str = "transactions_can_be_nested";
    setup_test_table(connection, TEST_NAME);
    fn get_count(connection: &mut TestConnection) -> i64 {
        count_test_table(connection, TEST_NAME)
    }

    let _ = connection.transaction::<(), _, _>(|connection| {
        diesel::sql_query(format!("INSERT INTO {TEST_NAME} DEFAULT VALUES"))
            .execute(connection)
            .unwrap();
        assert_eq!(1, get_count(connection));
        let _ = connection.transaction::<(), _, _>(|connection| {
            diesel::sql_query(format!("INSERT INTO {TEST_NAME} DEFAULT VALUES"))
                .execute(connection)
                .unwrap();
            assert_eq!(2, get_count(connection));
            Err(Error::RollbackTransaction)
        });
        assert_eq!(1, get_count(connection));
        let _ = connection.transaction::<(), Error, _>(|connection| {
            diesel::sql_query(format!("INSERT INTO {TEST_NAME} DEFAULT VALUES"))
                .execute(connection)
                .unwrap();
            assert_eq!(2, get_count(connection));
            Ok(())
        });
        assert_eq!(2, get_count(connection));
        Err(Error::RollbackTransaction)
    });
    assert_eq!(0, get_count(connection));

    drop_test_table(connection, TEST_NAME);
}

#[test]
fn test_transaction_always_rolls_back() {
    let connection = &mut connection_without_transaction();
    let test_name = "test_transaction_always_rolls_back";
    setup_test_table(connection, test_name);

    let result = connection.test_transaction::<_, Error, _>(|connection| {
        diesel::sql_query(format!("INSERT INTO {test_name} DEFAULT VALUES")).execute(connection)?;
        assert_eq!(1, count_test_table(connection, test_name));
        Ok("success")
    });
    assert_eq!(0, count_test_table(connection, test_name));
    assert_eq!("success", result);

    drop_test_table(connection, test_name);
}

#[test]
#[should_panic(expected = "Transaction did not succeed")]
fn test_transaction_panics_on_error() {
    let connection = &mut connection_without_transaction();
    connection.test_transaction::<(), _, _>(|_| Err(()));
}

fn setup_test_table(connection: &mut TestConnection, table_name: &str) {
    use crate::schema_dsl::*;
    create_table(table_name, (integer("id").primary_key().auto_increment(),))
        .execute(connection)
        .unwrap();
}

fn drop_test_table(connection: &mut TestConnection, table_name: &str) {
    diesel::sql_query(format!("DROP TABLE {table_name}"))
        .execute(connection)
        .unwrap();
}

fn count_test_table(connection: &mut TestConnection, table_name: &str) -> i64 {
    use diesel::dsl::sql;
    select(sql::<sql_types::BigInt>(&format!(
        "COUNT(*) FROM {table_name}"
    )))
    .first(connection)
    .unwrap()
}
