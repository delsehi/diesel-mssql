use super::query_builder::MssqlQueryBuilder;
use diesel::backend::*;
use diesel::query_builder::bind_collector::RawBytesBindCollector;
use diesel::sql_types::TypeMetadata;
use tiberius::ColumnData;
use tiberius::ColumnType;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Mssql;

impl Backend for Mssql {
    type QueryBuilder = MssqlQueryBuilder;
    type RawValue<'a> = ColumnData<'a>;
    type BindCollector<'a> = RawBytesBindCollector<Mssql>;
}

impl TrustedBackend for Mssql {}
impl DieselReserveSpecialization for Mssql {}

pub struct MssqlSelectSyntax;

impl SqlDialect for Mssql {
    type ReturningClause = sql_dialect::returning_clause::DoesNotSupportReturningClause;
    type OnConflictClause = sql_dialect::on_conflict_clause::DoesNotSupportOnConflictClause;
    type InsertWithDefaultKeyword =
        sql_dialect::default_keyword_for_insert::DoesNotSupportDefaultKeyword;
    type BatchInsertSupport = sql_dialect::batch_insert_support::DoesNotSupportBatchInsert;
    type ConcatClause = sql_dialect::concat_clause::ConcatWithPipesClause;
    type DefaultValueClauseForInsert = sql_dialect::default_value_clause::AnsiDefaultValueClause;
    type EmptyFromClauseSyntax = sql_dialect::from_clause_syntax::AnsiSqlFromClauseSyntax;
    type ExistsSyntax = sql_dialect::exists_syntax::AnsiSqlExistsSyntax;
    type ArrayComparison = sql_dialect::array_comparison::AnsiSqlArrayComparison;
    type SelectStatementSyntax = MssqlSelectSyntax;
    type AliasSyntax = sql_dialect::alias_syntax::AsAliasSyntax;
}

impl TypeMetadata for Mssql {
    type TypeMetadata = ColumnType;

    type MetadataLookup = ();
}
