# Diesel Tiberius

A SQL Server third-party backend for Diesel based on Tiberius.

Work in progress.

## Contributing

To run the tests, create a local instance of SQL Server with Docker or Podman:

```
docker run -e "ACCEPT_EULA=Y" -e "MSSQL_SA_PASSWORD=AStrongPassword123!" \
   -p 1433:1433 --name local_mssql \
   -d \
   mcr.microsoft.com/mssql/server:2022-latest
```

You might have to connect to it and create a database.

```
docker exec -it local_mssql /opt/mssql-tools18/bin/sqlcmd -S localhost -C -U SA -P AStrongPassword123! -Q "CREATE DATABASE diesel_mssql_db"
```

The tests are currently in a separate crate in the workspace. To run them:

```
cd tests
cargo test
```

Make a copy of `.env.sample` in the tests crate and rename it to `.env` and edit the values.

## Known issues and limitations

All tests as of this commit are passing except the ones marked with
ignore and an explanation.

There are many issues left to be fixed.

- The test `transaction_executes_fn_in_a_sql_transaction` is ignored because it
  takes forever. The statements seem to be blocking in SQL Server. Trying the same
  queries in Azure Data Studio yields the same results.
- OUTPUT (returning clause) is not yet implemented.
- NULL values are bound with VARBINARY by default.
- Selecting with equality operators on column values will error. The syntax is not
  supported in SQL Server and has to be rewritten to something like

```sql
SELECT CASE WHEN a = b THEN CONVERT(BIT, 1) ELSE CONVERT(BIT, 0) END AS c FROM d;
```

- Using the `offset` or `order` DSL methods on boxed queries will fail.
  If you need syntax like in the test `boxed_queries::boxed_queries_implement_offset_dsl`, `limit_offset::boxed_limit` or `limit_offset::boxed_offset` this will probably require
  Diesel to add order clauses to limit-offset upstreams.
