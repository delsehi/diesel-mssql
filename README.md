# Diesel Tiberius

A SQL Server third-party backend for Diesel based on Tiberius.

Work in progress.


## Known issues and limitations 

* The test `transaction_executes_fn_in_a_sql_transaction` is ignored because it 
takes forever. The statements seem to be blocking in SQL Server. Trying the same 
queries in Azure Data Studio yields the same results. 
* OUTPUT (returning clause) is not yet implemented. 
* NULL values are bound with VARBINARY by default. 
* Selecting with equality operators on column values will error. The syntax is not 
supported in SQL Server and has to be rewritten to something like 
```sql
SELECT CASE WHEN a = b THEN CONVERT(BIT, 1) ELSE CONVERT(BIT, 0) END AS c FROM d;
```