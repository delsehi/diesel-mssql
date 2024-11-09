use super::backend::MssqlSelectSyntax;
use diesel::query_builder::LimitClause;
use diesel::query_builder::LimitOffsetClause;
use diesel::query_builder::NoOffsetClause;
use diesel::query_builder::OffsetClause;
use diesel::query_builder::QueryBuilder;
use diesel::query_builder::QueryFragment;
use diesel::query_builder::SelectStatement;

use super::backend::Mssql;

pub struct MssqlQueryBuilder {
    query: String,
    bind_idx: u32,
}

impl MssqlQueryBuilder {
    pub fn new() -> Self {
        MssqlQueryBuilder {
            query: String::new(),
            bind_idx: 1,
        }
    }
}

impl Default for MssqlQueryBuilder {
    fn default() -> Self {
        MssqlQueryBuilder::new()
    }
}

impl QueryBuilder<Mssql> for MssqlQueryBuilder {
    fn push_sql(&mut self, sql: &str) {
        self.query.push_str(sql);
    }

    fn push_identifier(&mut self, identifier: &str) -> diesel::QueryResult<()> {
        self.push_sql("[");
        self.push_sql(&identifier);
        self.push_sql("]");
        Ok(())
    }

    fn push_bind_param(&mut self) {
        let param = format!("@P{}", self.bind_idx);
        self.bind_idx += 1;
        self.query.push_str(&param);
    }

    fn finish(self) -> String {
        self.query
    }
}

impl<F, S, D, W, O, LOf, G, H, LC> QueryFragment<Mssql, MssqlSelectSyntax>
    for SelectStatement<F, S, D, W, O, LOf, G, H, LC>
where
    F: QueryFragment<Mssql>,
    S: QueryFragment<Mssql>,
    D: QueryFragment<Mssql>,
    W: QueryFragment<Mssql>,
    O: QueryFragment<Mssql>,
    LOf: QueryFragment<Mssql>,
    G: QueryFragment<Mssql>,
    H: QueryFragment<Mssql>,
    LC: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql("SELECT ");
        self.distinct.walk_ast(out.reborrow())?;
        self.limit_offset.walk_ast(out.reborrow())?;
        self.select.walk_ast(out.reborrow())?;
        self.from.walk_ast(out.reborrow())?;
        self.where_clause.walk_ast(out.reborrow())?;
        self.group_by.walk_ast(out.reborrow())?;
        self.having.walk_ast(out.reborrow())?;
        self.order.walk_ast(out.reborrow())?;
        Ok(())
    }
}

impl<L, O> QueryFragment<Mssql> for LimitOffsetClause<LimitClause<L>, OffsetClause<O>>
where
    L: QueryFragment<Mssql>,
    O: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        out.push_sql(" TOP ");
        self.limit_clause.0.walk_ast(out.reborrow())?;

        self.offset_clause.walk_ast(out.reborrow())?;
        Ok(())
    }
}

impl<L> QueryFragment<Mssql> for LimitOffsetClause<LimitClause<L>, NoOffsetClause>
where
    L: QueryFragment<Mssql>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: diesel::query_builder::AstPass<'_, 'b, Mssql>,
    ) -> diesel::QueryResult<()> {
        // TOP() syntax needed if it's a parameter as in Diesel.
        // Diesel uses a varchar it seems, need to cast this to an integer for SQL Server.
        out.push_sql(" TOP(CAST(");
        self.limit_clause.0.walk_ast(out.reborrow())?;
        out.push_sql(" AS INT)) ");
        Ok(())
    }
}

// impl<E> QueryFragment<Mssql> for diesel::query_builder::LimitClause<E>
// where
//     diesel::query_builder::LimitClause<E>: QueryFragment<Mssql>,
// {
//     fn walk_ast<'b>(
//         &'b self,
//         pass: diesel::query_builder::AstPass<'_, 'b, Mssql>,
//     ) -> diesel::QueryResult<()> {
//         todo!()
//     }
// }
