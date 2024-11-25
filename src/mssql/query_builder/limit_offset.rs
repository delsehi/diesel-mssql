use super::super::backend::Mssql;
use diesel::query_builder::*;

impl QueryFragment<Mssql> for LimitOffsetClause<NoLimitClause, NoOffsetClause> {
    fn walk_ast<'b>(&'b self, _out: AstPass<'_, 'b, Mssql>) -> diesel::QueryResult<()> {
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
        out.push_sql(" TOP(");
        self.limit_clause.0.walk_ast(out.reborrow())?;
        out.push_sql(") ");
        Ok(())
    }
}
